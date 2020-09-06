import * as minidecaf from "minidecaf-frontend";
import {memory} from "../pkg/minidecaf_frontend_bg";

minidecaf.init_panic_hook();

window.set_err = function (msg) {
  window.outputCM.setValue(msg);
};

const init_mem = new Int32Array(new Int32Array(memory.buffer));

function run(act) {
  try {
    new Int32Array(memory.buffer).set(init_mem);
    const input = window.inputCM.getValue();
    window.outputCM.setValue(minidecaf.work(act, input));
  } catch (e) {
  }
}

document.getElementById('btnlex').onclick = () => run('lex');
document.getElementById('btnast').onclick = () => run('ast');
document.getElementById('btnir').onclick = () => run('ir');
document.getElementById('btnasm').onclick = () => run('asm');