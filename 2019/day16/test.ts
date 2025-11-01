function *genPattern(n: number, totalLen: number = 100): Generator<number, void, unknown> {
    let count = 0;
    for (let o = 0; true; o++) {
        // const outputNum = Math.round(Math.sin(o * Math.PI / 2));
        const outputNum = o % 2;
        for (let i = 0; i < n; i++) {
            yield outputNum;
            if (++count >= totalLen) return;
        }
    }
}
// function genPattern(n: number)

function add(a: number[], b: number[]): number[] {
    return a.flatMap((v, i) => b[i] === undefined ? [] : [v + b[i]]);
}
function sub(a: number[], b: number[]): number[] {
    return a.flatMap((v, i) => b[i] === undefined ? [] : [v - b[i]]);
}

function display(l: number[]) {
    console.log(l.map(l => (l < 0 ? "-" : " ") + Math.abs(l).toString()).join(" "));
}

const patterns = Array<number[]>(20).fill([]).map((_, i) => [...genPattern(i + 1, 40)]);

for (let i = 0; i < 19; i++) {
    display(patterns[i]);
}

display(add(patterns[15], patterns[7]));
