import * as tprime from "./tprime";

const application = new tprime.Application();

const tick = () => {
    for (let i = 0; i < 16; i++) {
        application.tick();
    }
    requestAnimationFrame(tick);
};

window.tprime = tprime;
window.application = application;

requestAnimationFrame(tick);
