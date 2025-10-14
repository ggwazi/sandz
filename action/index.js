const fetch = require('node-fetch');
(async()=>{
 const url = process.env.ORCHESTRA_URL;
 const flow = process.env.FLOW_FILE || 'flow.yaml';
 const res = await fetch(url+'/run',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({name:flow})});
 console.log(await res.text());
})();
