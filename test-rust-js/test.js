console.time('abc');
for(let i = 1; i < 50; i++){
    let a = cui(i)
    console.log(a)
}
console.timeEnd('abc');
function cui(n){
    if(n == 1) return 1;
    if(n == 2) return 2;

    return cui(n-1) + cui(n-2)
}