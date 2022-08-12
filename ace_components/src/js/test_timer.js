let a = 1;

const tick = () => {
    a += 1;
    ace.log("Tick " + a);
    ace.move_by(0.1, 0, 0);
}
