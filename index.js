import * as tprime from "./tprime";

const application = new tprime.Application();

const tick = () => {
    application.tick();
    // requestAnimationFrame(tick);
    setTimeout(tick, 1000);
};

window.tprime = tprime;
window.application = application;

requestAnimationFrame(tick);
