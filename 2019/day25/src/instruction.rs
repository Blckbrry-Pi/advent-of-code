macro_rules! opcode_def {
    (
        // Enum definition
        $(#[$($enum_attr:tt)*])* $vis:vis enum $enum_name:ident {
            // Variants
            $(
                // Variant args
                $(#[$($variant_attr:tt)*])*
                $opcode:ident [$opcode_int:literal] {
                    $($(#[$($arg_attr:tt)*])* $arg_type:tt $arg:ident),*
                } => 
                //Variant implementation
                    $(custom<$machine_binding_custom:ident, $data:ident>  $custom_expr:tt)?
                    $(<$($machine_binding:ident)?> $non_custom_expr:tt)?
            ,)+
        }
    ) => {
        $(#[$($enum_attr)*])*
        #[derive(Clone, Copy, PartialEq, Eq)]
        $vis enum $enum_name {
            $(
                $(#[$($variant_attr)*])*
                $opcode {
                    $(
                        $arg: $crate::instruction::Addr,
                    )*
                },
            )+
        }

        impl $enum_name {
            pub fn parse(machine: &$crate::machine::Machine, data: &[isize]) -> Option<Self> {
                if machine.pc >= data.len() { return None }
                let opcode = data[machine.pc];
                let mode_0 = $crate::instruction::Mode::from_int(opcode / 100 % 10);
                let mode_1 = $crate::instruction::Mode::from_int(opcode / 1000 % 10);
                let mode_2 = $crate::instruction::Mode::from_int(opcode / 10000 % 10);
                let modes = [mode_0, mode_1, mode_2];
                // println!("{opcode}");
                match opcode % 100 {
                    $($opcode_int => {
                        #[allow(unused_mut, unused_variables)]
                        let mut idx = 0;
                        $(
                            let $arg = Addr { arg: data[machine.pc+idx+1], mode: modes[idx] };

                            #[allow(unused_assignments)]
                            { idx += 1 }
                        )*
                        Some(Self::$opcode {
                            $($arg,)*
                        })
                    },)+
                    _ => None,
                }

            }

            pub fn exec(&self, machine: &mut $crate::machine::Machine, data: &mut [isize]) {
                match *self {
                    $(Self::$opcode { $($arg,)* } => {
                        #[allow(unused_variables)]
                        let pc_advance = ["opcode", $(stringify!($arg),)*].len();
                        $(
                            $crate::instruction::opcode_def!(@impl arg $arg_type $arg <machine, data>);
                        )*
                        $(let $machine_binding_custom = &mut *machine;)?
                        $($(let $machine_binding = &mut *machine;)?)?
                        $(
                            let $data = data;
                            #[allow(unused_braces)]
                            $custom_expr
                        )?
                        $(
                            #[allow(unused_braces)]
                            $non_custom_expr
                            machine.pc += pc_advance;
                        )?
                    })+
                }
            }

            pub fn size(&self) -> usize {
                match self {
                    $(Self::$opcode { .. } => ["opcode", $(stringify!($arg),)*].len(),)* 
                }
            }
        }

        impl std::fmt::Debug for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match *self {
                    $(Self::$opcode { $($arg,)* } => {
                        // #[allow(dead_code)]
                        // #[derive(Debug)]
                        // struct $opcode {
                        //     $($arg: $crate::instruction::Addr,)*
                        // }
                        write!(f, "{} ", stringify!($opcode))?;
                        $(write!(f, "{:?}, ", $arg)?;)*
                        Ok(())
                    },)*
                }
            }
        }
    };

    (@impl arg in $arg:ident <$machine:ident, $data:ident>) => {
        let $arg = {
            if $machine.debug {
                println!("got {}: {}", stringify!($arg), $arg.get($data, $machine));
            }
            $arg.get($data, $machine)
        };
    };
    (@impl arg out $arg:ident <$machine:ident, $data:ident>) => {
        let mut $arg = $arg.set_fn($machine);
        let mut $arg = move |val| $arg(val, $data);
    };
    (@impl arg raw $arg:ident <$machine:ident, $data:ident>) => {};
}

use std::fmt::Debug;

pub (crate) use opcode_def;

use crate::machine::Machine;

opcode_def!(
    pub enum Instruction {
        Add [1] { in a, in b, out to } => <>{ to(a + b) },
        Mul [2] { in a, in b, out to } => <>{ to(a * b) },
        Inp [3] { out to } => <machine> {
            let input = machine.input();
            if machine.debug {
                println!("char: {input} ({})", input as u8 as char);
            }
            to(input)
        },
        Out [4] { in val } => <machine> { machine.output(val) },
        Jit [5] { in cond, in addr } => custom<machine, _data> {
            if machine.debug {
                println!("tru: cond: {cond}, addr: {addr}");
            }
            if cond != 0 {
                machine.pc = addr as usize;
            } else {
                machine.pc += 3;
            }
        },
        Jif [6] { in cond, in addr } => custom<machine, _data> {
            if machine.debug {
                println!("fls: cond: {cond}, addr: {addr}");
            }
            if cond == 0 {
                machine.pc = addr as usize;
            } else {
                machine.pc += 3;
            }
        },
        SLt [7] { in l, in r, out to } => <>{ to((l < r) as isize) },
        SEq [8] { in l, in r, out to } => <>{ to((l == r) as isize) },
        Rel [9] { in adj } => <machine>{
            machine.offset += adj;
        },

        Hlt [99] {} => <machine>{ machine.halt = true; },
    }
);


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    Position,
    Immediate,
    Relative
}
impl Mode {
    fn from_int(int: isize) -> Self {
        match int {
            0 => Self::Position,
            1 => Self::Immediate,
            2 => Self::Relative,
            _ => panic!("Invalid integer"),
        }
    }
    pub fn get(&self, arg: isize, data: &[isize], offset: isize) -> isize {
        // println!("{arg} ?+ {offset}");
        match self {
            Self::Position => data[arg as usize],
            Self::Immediate => arg,
            Self::Relative => data[(arg + offset) as usize],
        }
    }
    pub fn set(&self, arg: isize, val: isize, data: &mut [isize], offset: isize) {
        match self {
            Self::Position => data[arg as usize] = val,
            Self::Immediate => unimplemented!("Cannot set an immediate value"),
            Self::Relative => data[(arg + offset) as usize] = val,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Addr { pub arg: isize, pub mode: Mode }

impl Addr {
    pub fn get(&self, data: &[isize], machine: &Machine) -> isize {
        self.mode.get(self.arg, data, machine.offset)
    }
    pub fn set(&self, val: isize, data: &mut [isize], machine: &Machine) {
        self.mode.set(self.arg, val, data, machine.offset)
    }

    pub fn set_fn(&self, machine: &Machine) -> impl FnMut(isize, &mut [isize]) {
        let Self { arg, mode } = *self;
        let offset = machine.offset;
        move |val, data| mode.set(arg, val, data, offset)
    }
}

impl Debug for Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct ArgVal { val: isize }
        impl Debug for ArgVal {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if self.val >= 0 {
                    write!(f, "+0x{:04x}", self.val)
                } else {
                    write!(f, "-0x{:04x}", -self.val)
                }
            }
        }
        match self.mode {
            Mode::Position => write!(f, "[{:?}]", ArgVal { val: self.arg }),
            Mode::Immediate => write!(f, "{:?}", ArgVal { val: self.arg }),
            Mode::Relative => write!(f, "[$rel + {:?}]", ArgVal { val: self.arg }),
        }
    }
}
