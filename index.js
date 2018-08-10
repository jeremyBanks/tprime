import * as tprime from "./tprime";

const application = new tprime.Application();

const tick = () => {
    application.tick();
    requestAnimationFrame(tick);
};

window.tprime = tprime;
window.application = application;

requestAnimationFrame(tick);
