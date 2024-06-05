
let timer;
self.addEventListener("message", event => {
	let seekTime = +event.data?.seekTime;
	const docVisible = event.data?.docVisible;

	if(!docVisible){
		timer = setInterval(()=>{
			seekTime += 1;
			self.postMessage({seekTime});
		}, 1000);
	}
	else{
		clearInterval(timer);
	}
});
