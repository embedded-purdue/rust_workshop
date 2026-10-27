'use strict';
function validateImage(image) {
  if (!image || image.magic !== 'TINYFLIGHT-1' || image.target !== 2 || image.dt !== .02 || !Array.isArray(image.samples) || image.samples.length !== 1000) throw Error('Wrong image format. Build with cargo run --bin firmware.');
  image.samples.forEach((s, i) => {
    if (!Array.isArray(s) || s.length !== 5 || !s.every(Number.isFinite) || Math.abs(s[0] - i * .02) > .0001 || s[1] < 0 || s[1] > 10000 || Math.abs(s[2]) > 10000 || s[3] < 0 || s[3] > 1 || s[4] !== (i >= 250 ? -1 : 0)) throw Error('Invalid telemetry sample ' + i);
  });
  if (image.sensor_samples !== undefined) {
    if (!Array.isArray(image.sensor_samples) || image.sensor_samples.length !== 1000) throw Error('Invalid sensor trace length.');
    image.sensor_samples.forEach((row, i) => {
      if (!Array.isArray(row) || row.length !== 4 || row[0] !== i || !row.slice(0,3).every(Number.isInteger) || row.slice(1,3).some(v => v < -2147483648 || v > 2147483647)) throw Error('Invalid sensor registers at sample ' + i);
      const stored = row[3];
      if (stored !== null && (!Array.isArray(stored) || stored.length !== 3 || !stored.every(Number.isFinite) || !Number.isInteger(stored[0]) || stored[0] < 0 || stored[0] > 65535)) throw Error('Invalid stored reading at sample ' + i);
    });
  }
  return image;
}
class FakeBoard {
  constructor() { this.mode = 'EMPTY'; this.image = null; }
  boot() { if (this.mode === 'FLASHING') throw Error('Wait for flashing to finish.'); this.mode = 'BOOT'; }
  begin(image) { if (this.mode !== 'BOOT') throw Error('Enter BOOT before flashing.'); validateImage(image); this.mode = 'FLASHING'; }
  commit(image) { if (this.mode !== 'FLASHING') throw Error('No flash in progress.'); this.image = validateImage(image); this.mode = 'READY'; }
  arm() { if (!this.image || !['READY', 'PAUSED'].includes(this.mode)) throw Error('Flash an image before arming.'); this.mode = 'ARMED'; }
  reset() { if (this.mode === 'FLASHING') throw Error('Wait for flashing to finish.'); this.mode = this.image ? 'READY' : 'EMPTY'; }
}
if (typeof module !== 'undefined') module.exports = { validateImage, FakeBoard };
if (typeof document !== 'undefined') {
  const $ = id => document.getElementById(id), board = new FakeBoard();
  let pending = null, index = 0, elapsed = 0, last = 0, generation = 0, frame = 0;
  const pinText = {'3V3':'Pin 1 / fictional 3.3 V supply. No wiring required.', GND:'Pin 2 / common reference for all signals.', LED:'Pin 3 / status LED. Rust flight playback turns it on while armed.', ALT:'Pin 4 / fake altitude sensor publishes signed millimeters at register 0x04. Decode, convert, and store before PID.', VEL:'Pin 5 / fake velocity sensor publishes signed mm/s at register 0x08. Negative means downward. Decode with i32::from_le_bytes.', BOOT:'Pin 10 / enters the fake loader. Use the Enter BOOT button before flashing.', M1:'Pin 6 / front-left motor PWM.', M2:'Pin 7 / front-right motor PWM.', M3:'Pin 8 / rear-right motor PWM.', M4:'Pin 9 / rear-left motor PWM.'};
  document.querySelectorAll('[data-pin]').forEach(b => b.onclick = () => { $('pininfo').textContent = pinText[b.dataset.pin] + (b.dataset.pin.startsWith('M') ? ' All four motors use the same duty for this altitude-only exercise.' : ''); });
  function log(message) { const line = document.createElement('div'); line.textContent = message; $('log').append(line); $('log').scrollTop = $('log').scrollHeight; }
  function guard(fn) { return async () => { try { $('error').textContent = ''; await fn(); } catch(e) { $('error').textContent = e.message; log('ERROR: ' + e.message); } sync(); }; }
  function sync() {
    $('mode').textContent = board.mode;
    $('led').classList.toggle('on', board.mode === 'ARMED');
    $('boot').disabled = board.mode === 'FLASHING';
    $('file').disabled = board.mode === 'FLASHING';
    $('flash').disabled = !pending || board.mode !== 'BOOT';
    $('arm').disabled = !['READY', 'PAUSED'].includes(board.mode);
    $('pause').disabled = board.mode !== 'ARMED';
    $('reset').disabled = board.mode === 'FLASHING';
    document.querySelectorAll('[data-pin^="M"]').forEach(p => p.classList.toggle('lit', board.mode === 'ARMED'));
  }
  function draw() {
    const ctx = $('plot').getContext('2d'), w = 1000, h = 220;
    ctx.clearRect(0,0,w,h); ctx.strokeStyle = '#e9ab76'; ctx.setLineDash([8,8]); ctx.beginPath(); ctx.moveTo(0,h/2); ctx.lineTo(w,h/2); ctx.stroke(); ctx.setLineDash([]);
    ctx.fillStyle='#aabdc9'; ctx.font='16px system-ui'; ctx.fillText('4 m',8,20); ctx.fillText('0 m',8,h-8); ctx.fillText('20 s',w-45,h-8);
    if (!board.image) return;
    ctx.strokeStyle='#80dce4';ctx.lineWidth=3;ctx.beginPath();
    board.image.samples.slice(0,index+1).forEach((s,i)=> { const x=s[0]/20*w,y=h-Math.min(4,s[1])/4*h; if(i===0)ctx.moveTo(x,y);else ctx.lineTo(x,y); });ctx.stroke();
  }
  function hexBytes(value, width) {
    const buffer = new ArrayBuffer(4); new DataView(buffer).setInt32(0,value,true);
    return [...new Uint8Array(buffer)].slice(0,width).map(b=>b.toString(16).padStart(2,'0').toUpperCase()).join(' ');
  }
  function render(disarmed=false) {
    const s = board.image ? board.image.samples[index] : [0,0,0,0,0];
    $('alt').textContent=s[1].toFixed(2)+' m';$('vel').textContent=s[2].toFixed(2)+' m/s';$('pwm').textContent=disarmed?'0%':(s[3]*100).toFixed(0)+'%';$('time').textContent=s[0].toFixed(1)+' s';
    const raw = board.image?.sensor_samples?.[index];
    $('regStatus').textContent = raw ? '01 / READY' : '—';
    $('regId').textContent = raw ? hexBytes(raw[0],2)+' / '+raw[0] : '—';
    $('regAlt').textContent = raw ? hexBytes(raw[1],4)+' / '+raw[1]+' mm' : '—';
    $('regVel').textContent = raw ? hexBytes(raw[2],4)+' / '+raw[2]+' mm/s' : '—';
    const stored = raw?.[3];
    $('stored').textContent = stored ? `self.latest = Some(SensorReading {\n  sample_id: ${stored[0]},\n  altitude_m: ${stored[1].toFixed(3)},\n  velocity_m_s: ${stored[2].toFixed(3)}\n})` : 'self.latest = None';
    $('sensorPath').textContent = !raw ? 'Build a new image to see the register trace.' : stored ? 'Stored readings above were passed into PID by the Rust controller.' : 'No stored reading. Complete controller.rs: read, decode, and store. Motor output is disabled.';
    $('drone').style.bottom=(8+Math.min(3.5,s[1])*80)+'px';$('drone').setAttribute('aria-label','Drone altitude '+s[1].toFixed(2)+' meters');draw();
  }
  $('boot').onclick=guard(()=>{board.boot();cancelAnimationFrame(frame);generation++;index=0;elapsed=0;render(true);log('BOOT pin asserted. Loader ready.');$('mission').textContent='Select your built image, then flash.';});
  $('file').onchange=guard(async()=>{pending=null;const file=$('file').files[0];if(!file)return;const token=++generation;if(file.size>250000)throw Error('Image too large (maximum 250 KB).');const text=await file.text();if(token!==generation)return;pending=validateImage(JSON.parse(text));$('image').textContent=file.name+' / '+file.size+' bytes / TF-01';log('Image format verified.');});
  $('flash').onclick=guard(async()=>{const image=pending;board.begin(image);sync();log('Erasing fake flash...');for(let p=0;p<=100;p+=5){$('progress').value=p;await new Promise(r=>setTimeout(r,35));}board.commit(image);index=0;elapsed=0;render(true);log('Copied image. Verified. Reboot into firmware.');$('mission').textContent='Ready. Arm to test your Rust controller.';});
  $('arm').onclick=guard(()=>{board.arm();last=performance.now();log('ARM: running flashed controller trace.');sync();frame=requestAnimationFrame(tick);});
  $('pause').onclick=guard(()=>{board.mode='PAUSED';cancelAnimationFrame(frame);$('mission').textContent='Paused. Arm resumes the same trace.';});
  $('reset').onclick=guard(()=>{board.reset();cancelAnimationFrame(frame);generation++;index=0;elapsed=0;render(true);log('Disarmed. Controller replay reset.');$('mission').textContent=board.image?'Ready for another run.':'Flash firmware to begin.';});
  function tick(now) {
    if(board.mode!=='ARMED')return;
    elapsed+=Math.min((now-last)/1000,.1)*Number($('speed').value);last=now;index=Math.min(999,Math.floor(elapsed/.02));render();
    $('mission').textContent=board.image.sensor_samples?.[index]?.[3] === null ? 'No sensor reading stored: motors off. Complete controller.rs, rebuild, and reflash.' : index>=250?'Gust active: −1 m/s². Can the controller recover?':'Climbing toward 2 m. Gust arrives at 5 s.';
    if(index===999){const pass=board.image.samples.slice(750).every(s=>Math.abs(2-s[1])<.1&&Math.abs(s[2])<.15);board.mode='READY';$('pwm').textContent='0%';$('mission').textContent=pass?'PASS: stable hover through the last 5 seconds.':'KEEP TUNING: last 5 seconds must stay within ±0.10 m and ±0.15 m/s.';log($('mission').textContent);elapsed=0;sync();return;}frame=requestAnimationFrame(tick);
  }
  log('TF-01 connected. Fake flash is empty.');sync();render(true);
}
