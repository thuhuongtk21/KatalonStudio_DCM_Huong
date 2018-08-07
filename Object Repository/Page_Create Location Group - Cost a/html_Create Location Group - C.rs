<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>html_Create Location Group - C</name>
   <tag></tag>
   <elementGuidId>65be1d94-4463-4b0c-b0cf-3b4fb4e721a5</elementGuidId>
   <selectorCollection>
      <entry>
         <key>BASIC</key>
         <value>//html[(text() = concat('






Create Location Group - Cost and Deals


	
(window.NREUM||(NREUM={})).loader_config={xpid:&quot;VQAPUFZSDBABVFlbBwcOUFM=&quot;};window.NREUM||(NREUM={}),__nr_require=function(t,e,n){function r(n){if(!e[n]){var o=e[n]={exports:{}};t[n][0].call(o.exports,function(e){var o=t[n][1][e];return r(o||e)},o,o.exports)}return e[n].exports}if(&quot;function&quot;==typeof __nr_require)return __nr_require;for(var o=0;o&lt;n.length;o++)r(n[o]);return r}({1:[function(t,e,n){function r(t){try{c.console&amp;&amp;console.log(t)}catch(e){}}var o,i=t(&quot;ee&quot;),a=t(20),c={};try{o=localStorage.getItem(&quot;__nr_flags&quot;).split(&quot;,&quot;),console&amp;&amp;&quot;function&quot;==typeof console.log&amp;&amp;(c.console=!0,o.indexOf(&quot;dev&quot;)!==-1&amp;&amp;(c.dev=!0),o.indexOf(&quot;nr_dev&quot;)!==-1&amp;&amp;(c.nrDev=!0))}catch(s){}c.nrDev&amp;&amp;i.on(&quot;internal-error&quot;,function(t){r(t.stack)}),c.dev&amp;&amp;i.on(&quot;fn-err&quot;,function(t,e,n){r(n.stack)}),c.dev&amp;&amp;(r(&quot;NR AGENT IN DEVELOPMENT MODE&quot;),r(&quot;flags: &quot;+a(c,function(t,e){return t}).join(&quot;, &quot;)))},{}],2:[function(t,e,n){function r(t,e,n,r,c){try{h?h-=1:o(c||new UncaughtException(t,e,n),!0)}catch(f){try{i(&quot;ierr&quot;,[f,s.now(),!0])}catch(d){}}return&quot;function&quot;==typeof u&amp;&amp;u.apply(this,a(arguments))}function UncaughtException(t,e,n){this.message=t||&quot;Uncaught error with no additional information&quot;,this.sourceURL=e,this.line=n}function o(t,e){var n=e?null:s.now();i(&quot;err&quot;,[t,n])}var i=t(&quot;handle&quot;),a=t(21),c=t(&quot;ee&quot;),s=t(&quot;loader&quot;),f=t(&quot;gos&quot;),u=window.onerror,d=!1,p=&quot;nr@seenError&quot;,h=0;s.features.err=!0,t(1),window.onerror=r;try{throw new Error}catch(l){&quot;stack&quot;in l&amp;&amp;(t(13),t(12),&quot;addEventListener&quot;in window&amp;&amp;t(6),s.xhrWrappable&amp;&amp;t(14),d=!0)}c.on(&quot;fn-start&quot;,function(t,e,n){d&amp;&amp;(h+=1)}),c.on(&quot;fn-err&quot;,function(t,e,n){d&amp;&amp;!n[p]&amp;&amp;(f(n,p,function(){return!0}),this.thrown=!0,o(n))}),c.on(&quot;fn-end&quot;,function(){d&amp;&amp;!this.thrown&amp;&amp;h>0&amp;&amp;(h-=1)}),c.on(&quot;internal-error&quot;,function(t){i(&quot;ierr&quot;,[t,s.now(),!0])})},{}],3:[function(t,e,n){t(&quot;loader&quot;).features.ins=!0},{}],4:[function(t,e,n){function r(){M++,S=y.hash,this[u]=b.now()}function o(){M--,y.hash!==S&amp;&amp;i(0,!0);var t=b.now();this[l]=~~this[l]+t-this[u],this[d]=t}function i(t,e){E.emit(&quot;newURL&quot;,[&quot;&quot;+y,e])}function a(t,e){t.on(e,function(){this[e]=b.now()})}var c=&quot;-start&quot;,s=&quot;-end&quot;,f=&quot;-body&quot;,u=&quot;fn&quot;+c,d=&quot;fn&quot;+s,p=&quot;cb&quot;+c,h=&quot;cb&quot;+s,l=&quot;jsTime&quot;,m=&quot;fetch&quot;,v=&quot;addEventListener&quot;,w=window,y=w.location,b=t(&quot;loader&quot;);if(w[v]&amp;&amp;b.xhrWrappable){var g=t(10),x=t(11),E=t(8),P=t(6),O=t(13),R=t(7),T=t(14),L=t(9),j=t(&quot;ee&quot;),N=j.get(&quot;tracer&quot;);t(15),b.features.spa=!0;var S,M=0;j.on(u,r),j.on(p,r),j.on(d,o),j.on(h,o),j.buffer([u,d,&quot;xhr-done&quot;,&quot;xhr-resolved&quot;]),P.buffer([u]),O.buffer([&quot;setTimeout&quot;+s,&quot;clearTimeout&quot;+c,u]),T.buffer([u,&quot;new-xhr&quot;,&quot;send-xhr&quot;+c]),R.buffer([m+c,m+&quot;-done&quot;,m+f+c,m+f+s]),E.buffer([&quot;newURL&quot;]),g.buffer([u]),x.buffer([&quot;propagate&quot;,p,h,&quot;executor-err&quot;,&quot;resolve&quot;+c]),N.buffer([u,&quot;no-&quot;+u]),L.buffer([&quot;new-jsonp&quot;,&quot;cb-start&quot;,&quot;jsonp-error&quot;,&quot;jsonp-end&quot;]),a(T,&quot;send-xhr&quot;+c),a(j,&quot;xhr-resolved&quot;),a(j,&quot;xhr-done&quot;),a(R,m+c),a(R,m+&quot;-done&quot;),a(L,&quot;new-jsonp&quot;),a(L,&quot;jsonp-end&quot;),a(L,&quot;cb-start&quot;),E.on(&quot;pushState-end&quot;,i),E.on(&quot;replaceState-end&quot;,i),w[v](&quot;hashchange&quot;,i,!0),w[v](&quot;load&quot;,i,!0),w[v](&quot;popstate&quot;,function(){i(0,M>1)},!0)}},{}],5:[function(t,e,n){function r(t){}if(window.performance&amp;&amp;window.performance.timing&amp;&amp;window.performance.getEntriesByType){var o=t(&quot;ee&quot;),i=t(&quot;handle&quot;),a=t(13),c=t(12),s=&quot;learResourceTimings&quot;,f=&quot;addEventListener&quot;,u=&quot;resourcetimingbufferfull&quot;,d=&quot;bstResource&quot;,p=&quot;resource&quot;,h=&quot;-start&quot;,l=&quot;-end&quot;,m=&quot;fn&quot;+h,v=&quot;fn&quot;+l,w=&quot;bstTimer&quot;,y=&quot;pushState&quot;,b=t(&quot;loader&quot;);b.features.stn=!0,t(8);var g=NREUM.o.EV;o.on(m,function(t,e){var n=t[0];n instanceof g&amp;&amp;(this.bstStart=b.now())}),o.on(v,function(t,e){var n=t[0];n instanceof g&amp;&amp;i(&quot;bst&quot;,[n,e,this.bstStart,b.now()])}),a.on(m,function(t,e,n){this.bstStart=b.now(),this.bstType=n}),a.on(v,function(t,e){i(w,[e,this.bstStart,b.now(),this.bstType])}),c.on(m,function(){this.bstStart=b.now()}),c.on(v,function(t,e){i(w,[e,this.bstStart,b.now(),&quot;requestAnimationFrame&quot;])}),o.on(y+h,function(t){this.time=b.now(),this.startPath=location.pathname+location.hash}),o.on(y+l,function(t){i(&quot;bstHist&quot;,[location.pathname+location.hash,this.startPath,this.time])}),f in window.performance&amp;&amp;(window.performance[&quot;c&quot;+s]?window.performance[f](u,function(t){i(d,[window.performance.getEntriesByType(p)]),window.performance[&quot;c&quot;+s]()},!1):window.performance[f](&quot;webkit&quot;+u,function(t){i(d,[window.performance.getEntriesByType(p)]),window.performance[&quot;webkitC&quot;+s]()},!1)),document[f](&quot;scroll&quot;,r,{passive:!0}),document[f](&quot;keypress&quot;,r,!1),document[f](&quot;click&quot;,r,!1)}},{}],6:[function(t,e,n){function r(t){for(var e=t;e&amp;&amp;!e.hasOwnProperty(u);)e=Object.getPrototypeOf(e);e&amp;&amp;o(e)}function o(t){c.inPlace(t,[u,d],&quot;-&quot;,i)}function i(t,e){return t[1]}var a=t(&quot;ee&quot;).get(&quot;events&quot;),c=t(23)(a,!0),s=t(&quot;gos&quot;),f=XMLHttpRequest,u=&quot;addEventListener&quot;,d=&quot;removeEventListener&quot;;e.exports=a,&quot;getPrototypeOf&quot;in Object?(r(document),r(window),r(f.prototype)):f.prototype.hasOwnProperty(u)&amp;&amp;(o(window),o(f.prototype)),a.on(u+&quot;-start&quot;,function(t,e){var n=t[1],r=s(n,&quot;nr@wrapped&quot;,function(){function t(){if(&quot;function&quot;==typeof n.handleEvent)return n.handleEvent.apply(n,arguments)}var e={object:t,&quot;function&quot;:n}[typeof n];return e?c(e,&quot;fn-&quot;,null,e.name||&quot;anonymous&quot;):n});this.wrapped=t[1]=r}),a.on(d+&quot;-start&quot;,function(t){t[1]=this.wrapped||t[1]})},{}],7:[function(t,e,n){function r(t,e,n){var r=t[e];&quot;function&quot;==typeof r&amp;&amp;(t[e]=function(){var t=r.apply(this,arguments);return o.emit(n+&quot;start&quot;,arguments,t),t.then(function(e){return o.emit(n+&quot;end&quot;,[null,e],t),e},function(e){throw o.emit(n+&quot;end&quot;,[e],t),e})})}var o=t(&quot;ee&quot;).get(&quot;fetch&quot;),i=t(20);e.exports=o;var a=window,c=&quot;fetch-&quot;,s=c+&quot;body-&quot;,f=[&quot;arrayBuffer&quot;,&quot;blob&quot;,&quot;json&quot;,&quot;text&quot;,&quot;formData&quot;],u=a.Request,d=a.Response,p=a.fetch,h=&quot;prototype&quot;;u&amp;&amp;d&amp;&amp;p&amp;&amp;(i(f,function(t,e){r(u[h],e,s),r(d[h],e,s)}),r(a,&quot;fetch&quot;,c),o.on(c+&quot;end&quot;,function(t,e){var n=this;e?e.clone().arrayBuffer().then(function(t){n.rxSize=t.byteLength,o.emit(c+&quot;done&quot;,[null,e],n)}):o.emit(c+&quot;done&quot;,[t],n)}))},{}],8:[function(t,e,n){var r=t(&quot;ee&quot;).get(&quot;history&quot;),o=t(23)(r);e.exports=r,o.inPlace(window.history,[&quot;pushState&quot;,&quot;replaceState&quot;],&quot;-&quot;)},{}],9:[function(t,e,n){function r(t){function e(){s.emit(&quot;jsonp-end&quot;,[],p),t.removeEventListener(&quot;load&quot;,e,!1),t.removeEventListener(&quot;error&quot;,n,!1)}function n(){s.emit(&quot;jsonp-error&quot;,[],p),s.emit(&quot;jsonp-end&quot;,[],p),t.removeEventListener(&quot;load&quot;,e,!1),t.removeEventListener(&quot;error&quot;,n,!1)}var r=t&amp;&amp;&quot;string&quot;==typeof t.nodeName&amp;&amp;&quot;script&quot;===t.nodeName.toLowerCase();if(r){var o=&quot;function&quot;==typeof t.addEventListener;if(o){var a=i(t.src);if(a){var u=c(a),d=&quot;function&quot;==typeof u.parent[u.key];if(d){var p={};f.inPlace(u.parent,[u.key],&quot;cb-&quot;,p),t.addEventListener(&quot;load&quot;,e,!1),t.addEventListener(&quot;error&quot;,n,!1),s.emit(&quot;new-jsonp&quot;,[t.src],p)}}}}}function o(){return&quot;addEventListener&quot;in window}function i(t){var e=t.match(u);return e?e[1]:null}function a(t,e){var n=t.match(p),r=n[1],o=n[3];return o?a(o,e[r]):e[r]}function c(t){var e=t.match(d);return e&amp;&amp;e.length>=3?{key:e[2],parent:a(e[1],window)}:{key:t,parent:window}}var s=t(&quot;ee&quot;).get(&quot;jsonp&quot;),f=t(23)(s);if(e.exports=s,o()){var u=/[?&amp;](?:callback|cb)=([^&amp;#]+)/,d=/(.*)\.([^.]+)/,p=/^(\w+)(\.|$)(.*)$/,h=[&quot;appendChild&quot;,&quot;insertBefore&quot;,&quot;replaceChild&quot;];f.inPlace(HTMLElement.prototype,h,&quot;dom-&quot;),f.inPlace(HTMLHeadElement.prototype,h,&quot;dom-&quot;),f.inPlace(HTMLBodyElement.prototype,h,&quot;dom-&quot;),s.on(&quot;dom-start&quot;,function(t){r(t[0])})}},{}],10:[function(t,e,n){var r=t(&quot;ee&quot;).get(&quot;mutation&quot;),o=t(23)(r),i=NREUM.o.MO;e.exports=r,i&amp;&amp;(window.MutationObserver=function(t){return this instanceof i?new i(o(t,&quot;fn-&quot;)):i.apply(this,arguments)},MutationObserver.prototype=i.prototype)},{}],11:[function(t,e,n){function r(t){var e=a.context(),n=c(t,&quot;executor-&quot;,e),r=new f(n);return a.context(r).getCtx=function(){return e},a.emit(&quot;new-promise&quot;,[r,e],e),r}function o(t,e){return e}var i=t(23),a=t(&quot;ee&quot;).get(&quot;promise&quot;),c=i(a),s=t(20),f=NREUM.o.PR;e.exports=a,f&amp;&amp;(window.Promise=r,[&quot;all&quot;,&quot;race&quot;].forEach(function(t){var e=f[t];f[t]=function(n){function r(t){return function(){a.emit(&quot;propagate&quot;,[null,!o],i),o=o||!t}}var o=!1;s(n,function(e,n){Promise.resolve(n).then(r(&quot;all&quot;===t),r(!1))});var i=e.apply(f,arguments),c=f.resolve(i);return c}}),[&quot;resolve&quot;,&quot;reject&quot;].forEach(function(t){var e=f[t];f[t]=function(t){var n=e.apply(f,arguments);return t!==n&amp;&amp;a.emit(&quot;propagate&quot;,[t,!0],n),n}}),f.prototype[&quot;catch&quot;]=function(t){return this.then(null,t)},f.prototype=Object.create(f.prototype,{constructor:{value:r}}),s(Object.getOwnPropertyNames(f),function(t,e){try{r[e]=f[e]}catch(n){}}),a.on(&quot;executor-start&quot;,function(t){t[0]=c(t[0],&quot;resolve-&quot;,this),t[1]=c(t[1],&quot;resolve-&quot;,this)}),a.on(&quot;executor-err&quot;,function(t,e,n){t[1](n)}),c.inPlace(f.prototype,[&quot;then&quot;],&quot;then-&quot;,o),a.on(&quot;then-start&quot;,function(t,e){this.promise=e,t[0]=c(t[0],&quot;cb-&quot;,this),t[1]=c(t[1],&quot;cb-&quot;,this)}),a.on(&quot;then-end&quot;,function(t,e,n){this.nextPromise=n;var r=this.promise;a.emit(&quot;propagate&quot;,[r,!0],n)}),a.on(&quot;cb-end&quot;,function(t,e,n){a.emit(&quot;propagate&quot;,[n,!0],this.nextPromise)}),a.on(&quot;propagate&quot;,function(t,e,n){this.getCtx&amp;&amp;!e||(this.getCtx=function(){if(t instanceof Promise)var e=a.context(t);return e&amp;&amp;e.getCtx?e.getCtx():this})}),r.toString=function(){return&quot;&quot;+f})},{}],12:[function(t,e,n){var r=t(&quot;ee&quot;).get(&quot;raf&quot;),o=t(23)(r),i=&quot;equestAnimationFrame&quot;;e.exports=r,o.inPlace(window,[&quot;r&quot;+i,&quot;mozR&quot;+i,&quot;webkitR&quot;+i,&quot;msR&quot;+i],&quot;raf-&quot;),r.on(&quot;raf-start&quot;,function(t){t[0]=o(t[0],&quot;fn-&quot;)})},{}],13:[function(t,e,n){function r(t,e,n){t[0]=a(t[0],&quot;fn-&quot;,null,n)}function o(t,e,n){this.method=n,this.timerDuration=isNaN(t[1])?0:+t[1],t[0]=a(t[0],&quot;fn-&quot;,this,n)}var i=t(&quot;ee&quot;).get(&quot;timer&quot;),a=t(23)(i),c=&quot;setTimeout&quot;,s=&quot;setInterval&quot;,f=&quot;clearTimeout&quot;,u=&quot;-start&quot;,d=&quot;-&quot;;e.exports=i,a.inPlace(window,[c,&quot;setImmediate&quot;],c+d),a.inPlace(window,[s],s+d),a.inPlace(window,[f,&quot;clearImmediate&quot;],f+d),i.on(s+u,r),i.on(c+u,o)},{}],14:[function(t,e,n){function r(t,e){d.inPlace(e,[&quot;onreadystatechange&quot;],&quot;fn-&quot;,c)}function o(){var t=this,e=u.context(t);t.readyState>3&amp;&amp;!e.resolved&amp;&amp;(e.resolved=!0,u.emit(&quot;xhr-resolved&quot;,[],t)),d.inPlace(t,y,&quot;fn-&quot;,c)}function i(t){b.push(t),l&amp;&amp;(x?x.then(a):v?v(a):(E=-E,P.data=E))}function a(){for(var t=0;t&lt;b.length;t++)r([],b[t]);b.length&amp;&amp;(b=[])}function c(t,e){return e}function s(t,e){for(var n in t)e[n]=t[n];return e}t(6);var f=t(&quot;ee&quot;),u=f.get(&quot;xhr&quot;),d=t(23)(u),p=NREUM.o,h=p.XHR,l=p.MO,m=p.PR,v=p.SI,w=&quot;readystatechange&quot;,y=[&quot;onload&quot;,&quot;onerror&quot;,&quot;onabort&quot;,&quot;onloadstart&quot;,&quot;onloadend&quot;,&quot;onprogress&quot;,&quot;ontimeout&quot;],b=[];e.exports=u;var g=window.XMLHttpRequest=function(t){var e=new h(t);try{u.emit(&quot;new-xhr&quot;,[e],e),e.addEventListener(w,o,!1)}catch(n){try{u.emit(&quot;internal-error&quot;,[n])}catch(r){}}return e};if(s(h,g),g.prototype=h.prototype,d.inPlace(g.prototype,[&quot;open&quot;,&quot;send&quot;],&quot;-xhr-&quot;,c),u.on(&quot;send-xhr-start&quot;,function(t,e){r(t,e),i(e)}),u.on(&quot;open-xhr-start&quot;,r),l){var x=m&amp;&amp;m.resolve();if(!v&amp;&amp;!m){var E=1,P=document.createTextNode(E);new l(a).observe(P,{characterData:!0})}}else f.on(&quot;fn-end&quot;,function(t){t[0]&amp;&amp;t[0].type===w||a()})},{}],15:[function(t,e,n){function r(t){var e=this.params,n=this.metrics;if(!this.ended){this.ended=!0;for(var r=0;r&lt;d;r++)t.removeEventListener(u[r],this.listener,!1);if(!e.aborted){if(n.duration=a.now()-this.startTime,4===t.readyState){e.status=t.status;var i=o(t,this.lastSize);if(i&amp;&amp;(n.rxSize=i),this.sameOrigin){var s=t.getResponseHeader(&quot;X-NewRelic-App-Data&quot;);s&amp;&amp;(e.cat=s.split(&quot;, &quot;).pop())}}else e.status=0;n.cbTime=this.cbTime,f.emit(&quot;xhr-done&quot;,[t],t),c(&quot;xhr&quot;,[e,n,this.startTime])}}}function o(t,e){var n=t.responseType;if(&quot;json&quot;===n&amp;&amp;null!==e)return e;var r=&quot;arraybuffer&quot;===n||&quot;blob&quot;===n||&quot;json&quot;===n?t.response:t.responseText;return l(r)}function i(t,e){var n=s(e),r=t.params;r.host=n.hostname+&quot;:&quot;+n.port,r.pathname=n.pathname,t.sameOrigin=n.sameOrigin}var a=t(&quot;loader&quot;);if(a.xhrWrappable){var c=t(&quot;handle&quot;),s=t(16),f=t(&quot;ee&quot;),u=[&quot;load&quot;,&quot;error&quot;,&quot;abort&quot;,&quot;timeout&quot;],d=u.length,p=t(&quot;id&quot;),h=t(19),l=t(18),m=window.XMLHttpRequest;a.features.xhr=!0,t(14),f.on(&quot;new-xhr&quot;,function(t){var e=this;e.totalCbs=0,e.called=0,e.cbTime=0,e.end=r,e.ended=!1,e.xhrGuids={},e.lastSize=null,h&amp;&amp;(h>34||h&lt;10)||window.opera||t.addEventListener(&quot;progress&quot;,function(t){e.lastSize=t.loaded},!1)}),f.on(&quot;open-xhr-start&quot;,function(t){this.params={method:t[0]},i(this,t[1]),this.metrics={}}),f.on(&quot;open-xhr-end&quot;,function(t,e){&quot;loader_config&quot;in NREUM&amp;&amp;&quot;xpid&quot;in NREUM.loader_config&amp;&amp;this.sameOrigin&amp;&amp;e.setRequestHeader(&quot;X-NewRelic-ID&quot;,NREUM.loader_config.xpid)}),f.on(&quot;send-xhr-start&quot;,function(t,e){var n=this.metrics,r=t[0],o=this;if(n&amp;&amp;r){var i=l(r);i&amp;&amp;(n.txSize=i)}this.startTime=a.now(),this.listener=function(t){try{&quot;abort&quot;===t.type&amp;&amp;(o.params.aborted=!0),(&quot;load&quot;!==t.type||o.called===o.totalCbs&amp;&amp;(o.onloadCalled||&quot;function&quot;!=typeof e.onload))&amp;&amp;o.end(e)}catch(n){try{f.emit(&quot;internal-error&quot;,[n])}catch(r){}}};for(var c=0;c&lt;d;c++)e.addEventListener(u[c],this.listener,!1)}),f.on(&quot;xhr-cb-time&quot;,function(t,e,n){this.cbTime+=t,e?this.onloadCalled=!0:this.called+=1,this.called!==this.totalCbs||!this.onloadCalled&amp;&amp;&quot;function&quot;==typeof n.onload||this.end(n)}),f.on(&quot;xhr-load-added&quot;,function(t,e){var n=&quot;&quot;+p(t)+!!e;this.xhrGuids&amp;&amp;!this.xhrGuids[n]&amp;&amp;(this.xhrGuids[n]=!0,this.totalCbs+=1)}),f.on(&quot;xhr-load-removed&quot;,function(t,e){var n=&quot;&quot;+p(t)+!!e;this.xhrGuids&amp;&amp;this.xhrGuids[n]&amp;&amp;(delete this.xhrGuids[n],this.totalCbs-=1)}),f.on(&quot;addEventListener-end&quot;,function(t,e){e instanceof m&amp;&amp;&quot;load&quot;===t[0]&amp;&amp;f.emit(&quot;xhr-load-added&quot;,[t[1],t[2]],e)}),f.on(&quot;removeEventListener-end&quot;,function(t,e){e instanceof m&amp;&amp;&quot;load&quot;===t[0]&amp;&amp;f.emit(&quot;xhr-load-removed&quot;,[t[1],t[2]],e)}),f.on(&quot;fn-start&quot;,function(t,e,n){e instanceof m&amp;&amp;(&quot;onload&quot;===n&amp;&amp;(this.onload=!0),(&quot;load&quot;===(t[0]&amp;&amp;t[0].type)||this.onload)&amp;&amp;(this.xhrCbStart=a.now()))}),f.on(&quot;fn-end&quot;,function(t,e){this.xhrCbStart&amp;&amp;f.emit(&quot;xhr-cb-time&quot;,[a.now()-this.xhrCbStart,this.onload,e],e)})}},{}],16:[function(t,e,n){e.exports=function(t){var e=document.createElement(&quot;a&quot;),n=window.location,r={};e.href=t,r.port=e.port;var o=e.href.split(&quot;://&quot;);!r.port&amp;&amp;o[1]&amp;&amp;(r.port=o[1].split(&quot;/&quot;)[0].split(&quot;@&quot;).pop().split(&quot;:&quot;)[1]),r.port&amp;&amp;&quot;0&quot;!==r.port||(r.port=&quot;https&quot;===o[0]?&quot;443&quot;:&quot;80&quot;),r.hostname=e.hostname||n.hostname,r.pathname=e.pathname,r.protocol=o[0],&quot;/&quot;!==r.pathname.charAt(0)&amp;&amp;(r.pathname=&quot;/&quot;+r.pathname);var i=!e.protocol||&quot;:&quot;===e.protocol||e.protocol===n.protocol,a=e.hostname===document.domain&amp;&amp;e.port===n.port;return r.sameOrigin=i&amp;&amp;(!e.hostname||a),r}},{}],17:[function(t,e,n){function r(){}function o(t,e,n){return function(){return i(t,[f.now()].concat(c(arguments)),e?null:this,n),e?void 0:this}}var i=t(&quot;handle&quot;),a=t(20),c=t(21),s=t(&quot;ee&quot;).get(&quot;tracer&quot;),f=t(&quot;loader&quot;),u=NREUM;&quot;undefined&quot;==typeof window.newrelic&amp;&amp;(newrelic=u);var d=[&quot;setPageViewName&quot;,&quot;setCustomAttribute&quot;,&quot;setErrorHandler&quot;,&quot;finished&quot;,&quot;addToTrace&quot;,&quot;inlineHit&quot;,&quot;addRelease&quot;],p=&quot;api-&quot;,h=p+&quot;ixn-&quot;;a(d,function(t,e){u[e]=o(p+e,!0,&quot;api&quot;)}),u.addPageAction=o(p+&quot;addPageAction&quot;,!0),u.setCurrentRouteName=o(p+&quot;routeName&quot;,!0),e.exports=newrelic,u.interaction=function(){return(new r).get()};var l=r.prototype={createTracer:function(t,e){var n={},r=this,o=&quot;function&quot;==typeof e;return i(h+&quot;tracer&quot;,[f.now(),t,n],r),function(){if(s.emit((o?&quot;&quot;:&quot;no-&quot;)+&quot;fn-start&quot;,[f.now(),r,o],n),o)try{return e.apply(this,arguments)}catch(t){throw s.emit(&quot;fn-err&quot;,[arguments,this,t],n),t}finally{s.emit(&quot;fn-end&quot;,[f.now()],n)}}}};a(&quot;setName,setAttribute,save,ignore,onEnd,getContext,end,get&quot;.split(&quot;,&quot;),function(t,e){l[e]=o(h+e)}),newrelic.noticeError=function(t){&quot;string&quot;==typeof t&amp;&amp;(t=new Error(t)),i(&quot;err&quot;,[t,f.now()])}},{}],18:[function(t,e,n){e.exports=function(t){if(&quot;string&quot;==typeof t&amp;&amp;t.length)return t.length;if(&quot;object&quot;==typeof t){if(&quot;undefined&quot;!=typeof ArrayBuffer&amp;&amp;t instanceof ArrayBuffer&amp;&amp;t.byteLength)return t.byteLength;if(&quot;undefined&quot;!=typeof Blob&amp;&amp;t instanceof Blob&amp;&amp;t.size)return t.size;if(!(&quot;undefined&quot;!=typeof FormData&amp;&amp;t instanceof FormData))try{return JSON.stringify(t).length}catch(e){return}}}},{}],19:[function(t,e,n){var r=0,o=navigator.userAgent.match(/Firefox[\/\s](\d+\.\d+)/);o&amp;&amp;(r=+o[1]),e.exports=r},{}],20:[function(t,e,n){function r(t,e){var n=[],r=&quot;&quot;,i=0;for(r in t)o.call(t,r)&amp;&amp;(n[i]=e(r,t[r]),i+=1);return n}var o=Object.prototype.hasOwnProperty;e.exports=r},{}],21:[function(t,e,n){function r(t,e,n){e||(e=0),&quot;undefined&quot;==typeof n&amp;&amp;(n=t?t.length:0);for(var r=-1,o=n-e||0,i=Array(o&lt;0?0:o);++r&lt;o;)i[r]=t[e+r];return i}e.exports=r},{}],22:[function(t,e,n){e.exports={exists:&quot;undefined&quot;!=typeof window.performance&amp;&amp;window.performance.timing&amp;&amp;&quot;undefined&quot;!=typeof window.performance.timing.navigationStart}},{}],23:[function(t,e,n){function r(t){return!(t&amp;&amp;t instanceof Function&amp;&amp;t.apply&amp;&amp;!t[a])}var o=t(&quot;ee&quot;),i=t(21),a=&quot;nr@original&quot;,c=Object.prototype.hasOwnProperty,s=!1;e.exports=function(t,e){function n(t,e,n,o){function nrWrapper(){var r,a,c,s;try{a=this,r=i(arguments),c=&quot;function&quot;==typeof n?n(r,a):n||{}}catch(f){p([f,&quot;&quot;,[r,a,o],c])}u(e+&quot;start&quot;,[r,a,o],c);try{return s=t.apply(a,r)}catch(d){throw u(e+&quot;err&quot;,[r,a,d],c),d}finally{u(e+&quot;end&quot;,[r,a,s],c)}}return r(t)?t:(e||(e=&quot;&quot;),nrWrapper[a]=t,d(t,nrWrapper),nrWrapper)}function f(t,e,o,i){o||(o=&quot;&quot;);var a,c,s,f=&quot;-&quot;===o.charAt(0);for(s=0;s&lt;e.length;s++)c=e[s],a=t[c],r(a)||(t[c]=n(a,f?c+o:o,i,c))}function u(n,r,o){if(!s||e){var i=s;s=!0;try{t.emit(n,r,o,e)}catch(a){p([a,n,r,o])}s=i}}function d(t,e){if(Object.defineProperty&amp;&amp;Object.keys)try{var n=Object.keys(t);return n.forEach(function(n){Object.defineProperty(e,n,{get:function(){return t[n]},set:function(e){return t[n]=e,e}})}),e}catch(r){p([r])}for(var o in t)c.call(t,o)&amp;&amp;(e[o]=t[o]);return e}function p(e){try{t.emit(&quot;internal-error&quot;,e)}catch(n){}}return t||(t=o),n.inPlace=f,n.flag=a,n}},{}],ee:[function(t,e,n){function r(){}function o(t){function e(t){return t&amp;&amp;t instanceof r?t:t?s(t,c,i):i()}function n(n,r,o,i){if(!p.aborted||i){t&amp;&amp;t(n,r,o);for(var a=e(o),c=l(n),s=c.length,f=0;f&lt;s;f++)c[f].apply(a,r);var d=u[y[n]];return d&amp;&amp;d.push([b,n,r,a]),a}}function h(t,e){w[t]=l(t).concat(e)}function l(t){return w[t]||[]}function m(t){return d[t]=d[t]||o(n)}function v(t,e){f(t,function(t,n){e=e||&quot;feature&quot;,y[n]=e,e in u||(u[e]=[])})}var w={},y={},b={on:h,emit:n,get:m,listeners:l,context:e,buffer:v,abort:a,aborted:!1};return b}function i(){return new r}function a(){(u.api||u.feature)&amp;&amp;(p.aborted=!0,u=p.backlog={})}var c=&quot;nr@context&quot;,s=t(&quot;gos&quot;),f=t(20),u={},d={},p=e.exports=o();p.backlog=u},{}],gos:[function(t,e,n){function r(t,e,n){if(o.call(t,e))return t[e];var r=n();if(Object.defineProperty&amp;&amp;Object.keys)try{return Object.defineProperty(t,e,{value:r,writable:!0,enumerable:!1}),r}catch(i){}return t[e]=r,r}var o=Object.prototype.hasOwnProperty;e.exports=r},{}],handle:[function(t,e,n){function r(t,e,n,r){o.buffer([t],r),o.emit(t,e,n)}var o=t(&quot;ee&quot;).get(&quot;handle&quot;);e.exports=r,r.ee=o},{}],id:[function(t,e,n){function r(t){var e=typeof t;return!t||&quot;object&quot;!==e&amp;&amp;&quot;function&quot;!==e?-1:t===window?0:a(t,i,function(){return o++})}var o=1,i=&quot;nr@id&quot;,a=t(&quot;gos&quot;);e.exports=r},{}],loader:[function(t,e,n){function r(){if(!x++){var t=g.info=NREUM.info,e=p.getElementsByTagName(&quot;script&quot;)[0];if(setTimeout(u.abort,3e4),!(t&amp;&amp;t.licenseKey&amp;&amp;t.applicationID&amp;&amp;e))return u.abort();f(y,function(e,n){t[e]||(t[e]=n)}),s(&quot;mark&quot;,[&quot;onload&quot;,a()+g.offset],null,&quot;api&quot;);var n=p.createElement(&quot;script&quot;);n.src=&quot;https://&quot;+t.agent,e.parentNode.insertBefore(n,e)}}function o(){&quot;complete&quot;===p.readyState&amp;&amp;i()}function i(){s(&quot;mark&quot;,[&quot;domContent&quot;,a()+g.offset],null,&quot;api&quot;)}function a(){return E.exists&amp;&amp;performance.now?Math.round(performance.now()):(c=Math.max((new Date).getTime(),c))-g.offset}var c=(new Date).getTime(),s=t(&quot;handle&quot;),f=t(20),u=t(&quot;ee&quot;),d=window,p=d.document,h=&quot;addEventListener&quot;,l=&quot;attachEvent&quot;,m=d.XMLHttpRequest,v=m&amp;&amp;m.prototype;NREUM.o={ST:setTimeout,SI:d.setImmediate,CT:clearTimeout,XHR:m,REQ:d.Request,EV:d.Event,PR:d.Promise,MO:d.MutationObserver};var w=&quot;&quot;+location,y={beacon:&quot;bam.nr-data.net&quot;,errorBeacon:&quot;bam.nr-data.net&quot;,agent:&quot;js-agent.newrelic.com/nr-spa-1071.min.js&quot;},b=m&amp;&amp;v&amp;&amp;v[h]&amp;&amp;!/CriOS/.test(navigator.userAgent),g=e.exports={offset:c,now:a,origin:w,features:{},xhrWrappable:b};t(17),p[h]?(p[h](&quot;DOMContentLoaded&quot;,i,!1),d[h](&quot;load&quot;,r,!1)):(p[l](&quot;onreadystatechange&quot;,o),d[l](&quot;onload&quot;,r)),s(&quot;mark&quot;,[&quot;firstbyte&quot;,c],null,&quot;api&quot;);var x=0,E=t(22)},{}]},{},[&quot;loader&quot;,2,15,5,3,4]);



	

	

	

	

	

	

	

	

	

	

	

	

	

	





#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-elementInfoDiv {color: lightblue; padding: 5px}

	
		















    


    var contextPath = ' , &quot;'&quot; , '/DCM_UI' , &quot;'&quot; , ';
    var servletPath = ' , &quot;'&quot; , '/create-location-group' , &quot;'&quot; , ';
    var userId = ' , &quot;'&quot; , 'p165114' , &quot;'&quot; , ';
    var userRole = ' , &quot;'&quot; , 'ADMIN' , &quot;'&quot; , ';
    var userName = ' , &quot;'&quot; , 'Peters,Carrol' , &quot;'&quot; , ';
    var sessionTimeout = ' , &quot;'&quot; , '1800' , &quot;'&quot; , ';

    function formSubmit() {
        document.getElementById(&quot;logoutForm&quot;).submit();
    }


       Warning! Due to inactivity, your session will expire in 00:00:00. To extend your session another 30 minute(s), please press the Extend button.
    Extend


   Warning! Due to inactivity, your session has expired. Please save any data you may have entered before refreshing the page.


    
        
            
        
        Cost and Deals
    
    
        
            
                
                    
                    
                    
                
                Logged in as
                    
                    
                    
                    Peters,Carrol
                    
                    
                
                Sign Out
            
        
        
            
                Toggle navigation
                
                
                
            
        
        
            
                
                    Home
                    
                        
                            Switch To...
                            Announcements
                        
                    
                    
                
                
                    Offer
                    
                        
                            Create
                        
                        Deal Builder
                        Review
                        
                            Accept/Reject
                        
                        Search
                        
                            Maintain
                            Mass Upload
                        
                    
                
                Deals
                    
                        Search
                        Maintain
                        SODL
                            
                                Search &amp; Maintenance
                                Details
                                Transactions
                            
                        
                        Browse Deals and Costs
                    
                
                Cost
                    
                        Maintain
                        Browse Deals and Costs
                    
                
                Location Group
                    
                        
                            Create
                        
                        Search
                        
                            Maintain
                        
                    
                
                Items
                    
                        Cost Link Maintenance
                    
                
                
                    People
                        
                            User Preferences
                            Forward Alert and Share Authority
                        
                    
                
                
                
                Help
                
            
        
    
    
        
            
            
            
            
        
        
            
                
                    Help
                    
                
            

            
                
                    
                
                
                    
                
            
        


        
        
            
            	
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                    
                    HEB Enterprise Portal
                    Cost and Deals
                    Location Group
                    Create
                
                
                
                
                
                
                
                
                
            
        
    

            
            	
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                    
                    HEB Enterprise Portal
                    Cost and Deals
                    Location Group
                    Create
                
                
                
                
                
                
                
                
                
            
        

    
        
            
                User Setting
                ×
            
            
                
                    General
                    Custom Settings
                
                
                    
                        
                        
                            Dashboard
                            
                                 Show Information Alerts
                            
                            
                                 Activate &quot;Switch to&quot; User
                            
                            
                                Switch to User Id 
                                
                                    
                                
                                
                            
                        
                        
                            Available Partners
                            
                        
                        
                    
                    
                        
                        
                            Screen
                            
                                 Application 
                            
                            
                                 Screen Name 
                            
                            
                                 Tab Name 
                            
                        
                        
                            Misc.
                            
                                 Default Start Date to 
                                
                                 Days Prior to Today 
                            
                        
                        
                            Available Columns
                            
                        
                        
                    
                
                
                    OK
                    Cancel
                    Reset to Defaults
                
            
        
    


    var highlightMenuItem = ' , &quot;'&quot; , 'basket-create' , &quot;'&quot; , ';
    document.addEventListener(&quot;DOMContentLoaded&quot;, function(event) {
        if (highlightMenuItem !== ' , &quot;'&quot; , '' , &quot;'&quot; , ') {
            $(' , &quot;'&quot; , '*[data-highlight=&quot;' , &quot;'&quot; , ' + highlightMenuItem + ' , &quot;'&quot; , '&quot;]' , &quot;'&quot; , ').addClass(' , &quot;'&quot; , 'active' , &quot;'&quot; , ');
            $(' , &quot;'&quot; , '*[data-highlight=&quot;' , &quot;'&quot; , ' + highlightMenuItem + ' , &quot;'&quot; , '&quot;]' , &quot;'&quot; , ').closest(' , &quot;'&quot; , 'li.dropdown' , &quot;'&quot; , ').addClass(' , &quot;'&quot; , 'active' , &quot;'&quot; , ');
        }
    });


		
			
	
		CREATE LOCATION GROUP
		
			
				
			
			
				
			
			
				
			
			
				
			
			
				
			
		
	
	
		
		
		
			
				
					
						The following errors were encountered:
						
					
					
						
					
				
				
			
		
		
		
			
				Location Group # or Name
			
			
				
				
					
						Name:
						add new
					
					
						Owner:
						p165114 Peters,Carrol
					
					
						Domain:
						HEB - HEB ONLY PARTNERS
					
					
						Type:
						LOCATION
					
				

			
			
				
					
						Details for Location Group #
						
					
				
				

					
						
							
								Note: Required fields are marked with an asterisk (*)
							
							
								
								
									
										Group Name
									
									
										
										
											*
										
									
								
								
								
								
									
										Owner
									
									
										
										
											*
										
									
									
										Peters,Carrol
									
								
								
								
								
									
										Domain
									
									
										
											HEB - HEB ONLY PARTNERS - Select -HEB - HEB ONLY PARTNERSVEND - BOTH HEB AND VENDORS- Select -HEB - HEB ONLY PARTNERSVEND - BOTH HEB AND VENDORS
											
										
										
											*
										
									
								
								
								
								
									
										Type
									
									
										STORE - LOCATION
										
										
									
								

								
									
										Public
									
									
										
									
								

								
									
										Make Location Group usable by
									
									
										
									
									
										
									
								

							
						
						
							
								
									 
								
								
									
										Abbreviation
									
									
										
										
											*
										
									
								
								
								
									
										Description
									
									
										
											
										
									
								
								
								
									
										Level
									
									
										
											USER - USER DEFINED CGRP - COST GROUPCORP - CORPORATE DERIVEDUSER - USER DEFINEDCGRP - COST GROUPCORP - CORPORATE DERIVEDUSER - USER DEFINED
											
										
									
								
								
								
									
										No of Elements
									
									
										0
									
								
								
									
										Active
									
									
										
									
								
							
						

						
							
								
									 
								
								
									
										Public Comments
									
									
										
									
								
								
									
										Private Comments
									
									
										
									
								
							
							
								
									
										Created By
									
									
										
									
								
							
							
								
									
										Updated By
									
									
										
									
								
							
						
					
				
			
		
		
			
				
					
				
			
			
				
					
						
							Select
						
						
						
							
								
								
								
							
							
							
								
							
						
						
							All Stores
								
									
								00000 - 00001 - BR01 ELIZABETH00006 - STEPHENVILLE00009 - DONNA00013 - RIO GRANDE CITY-HWY83/S BRIDGE00014 - KYLE00016 - BURLESON - JOHN JONES/WILSHIRE00017 - ANGLETON00019 - LYTLE    FM 2790/IH3500020 - H53 FM 290/BARKER CYPRESS
							
						
						
							Page 1 of 39
							Total items 386
						
						
							12345Next >Last >>
						
					
				
				
					
						
					
					
						
					
					
						
					
				
				
					
						
							Add to Location Group
						
						
						
							
								
								
							
							
							
								
							
						
						
							
								
									
								
							
						
					
				
			
		
		

		
			
				
					
						
							Help
							
						
					

				
			
		

	
		var basketIdFromBasketSearch = &quot;0&quot;; //int
		var isCopyFromBasketSearch = &quot;&quot;;
	

		
	
	
	

window.NREUM||(NREUM={});NREUM.info={&quot;errorBeacon&quot;:&quot;bam.nr-data.net&quot;,&quot;licenseKey&quot;:&quot;107f4f8ae4&quot;,&quot;agent&quot;:&quot;&quot;,&quot;beacon&quot;:&quot;bam.nr-data.net&quot;,&quot;applicationTime&quot;:6,&quot;applicationID&quot;:&quot;108906967&quot;,&quot;transactionName&quot;:&quot;NVNSN0dTXkRTBkFZDQwZYxNHW15QcQpbRBANWlwGRx1TRVcEQVVPDllTAkFbX1kfAkdfFxIWGCRwZhk=&quot;,&quot;queueTime&quot;:0}


	

	

	

	

	

	

	

	

	

	

	

	

	

	

	

	

	

	

	

	

	

	



	

	

	


/html[1]') or . = concat('






Create Location Group - Cost and Deals


	
(window.NREUM||(NREUM={})).loader_config={xpid:&quot;VQAPUFZSDBABVFlbBwcOUFM=&quot;};window.NREUM||(NREUM={}),__nr_require=function(t,e,n){function r(n){if(!e[n]){var o=e[n]={exports:{}};t[n][0].call(o.exports,function(e){var o=t[n][1][e];return r(o||e)},o,o.exports)}return e[n].exports}if(&quot;function&quot;==typeof __nr_require)return __nr_require;for(var o=0;o&lt;n.length;o++)r(n[o]);return r}({1:[function(t,e,n){function r(t){try{c.console&amp;&amp;console.log(t)}catch(e){}}var o,i=t(&quot;ee&quot;),a=t(20),c={};try{o=localStorage.getItem(&quot;__nr_flags&quot;).split(&quot;,&quot;),console&amp;&amp;&quot;function&quot;==typeof console.log&amp;&amp;(c.console=!0,o.indexOf(&quot;dev&quot;)!==-1&amp;&amp;(c.dev=!0),o.indexOf(&quot;nr_dev&quot;)!==-1&amp;&amp;(c.nrDev=!0))}catch(s){}c.nrDev&amp;&amp;i.on(&quot;internal-error&quot;,function(t){r(t.stack)}),c.dev&amp;&amp;i.on(&quot;fn-err&quot;,function(t,e,n){r(n.stack)}),c.dev&amp;&amp;(r(&quot;NR AGENT IN DEVELOPMENT MODE&quot;),r(&quot;flags: &quot;+a(c,function(t,e){return t}).join(&quot;, &quot;)))},{}],2:[function(t,e,n){function r(t,e,n,r,c){try{h?h-=1:o(c||new UncaughtException(t,e,n),!0)}catch(f){try{i(&quot;ierr&quot;,[f,s.now(),!0])}catch(d){}}return&quot;function&quot;==typeof u&amp;&amp;u.apply(this,a(arguments))}function UncaughtException(t,e,n){this.message=t||&quot;Uncaught error with no additional information&quot;,this.sourceURL=e,this.line=n}function o(t,e){var n=e?null:s.now();i(&quot;err&quot;,[t,n])}var i=t(&quot;handle&quot;),a=t(21),c=t(&quot;ee&quot;),s=t(&quot;loader&quot;),f=t(&quot;gos&quot;),u=window.onerror,d=!1,p=&quot;nr@seenError&quot;,h=0;s.features.err=!0,t(1),window.onerror=r;try{throw new Error}catch(l){&quot;stack&quot;in l&amp;&amp;(t(13),t(12),&quot;addEventListener&quot;in window&amp;&amp;t(6),s.xhrWrappable&amp;&amp;t(14),d=!0)}c.on(&quot;fn-start&quot;,function(t,e,n){d&amp;&amp;(h+=1)}),c.on(&quot;fn-err&quot;,function(t,e,n){d&amp;&amp;!n[p]&amp;&amp;(f(n,p,function(){return!0}),this.thrown=!0,o(n))}),c.on(&quot;fn-end&quot;,function(){d&amp;&amp;!this.thrown&amp;&amp;h>0&amp;&amp;(h-=1)}),c.on(&quot;internal-error&quot;,function(t){i(&quot;ierr&quot;,[t,s.now(),!0])})},{}],3:[function(t,e,n){t(&quot;loader&quot;).features.ins=!0},{}],4:[function(t,e,n){function r(){M++,S=y.hash,this[u]=b.now()}function o(){M--,y.hash!==S&amp;&amp;i(0,!0);var t=b.now();this[l]=~~this[l]+t-this[u],this[d]=t}function i(t,e){E.emit(&quot;newURL&quot;,[&quot;&quot;+y,e])}function a(t,e){t.on(e,function(){this[e]=b.now()})}var c=&quot;-start&quot;,s=&quot;-end&quot;,f=&quot;-body&quot;,u=&quot;fn&quot;+c,d=&quot;fn&quot;+s,p=&quot;cb&quot;+c,h=&quot;cb&quot;+s,l=&quot;jsTime&quot;,m=&quot;fetch&quot;,v=&quot;addEventListener&quot;,w=window,y=w.location,b=t(&quot;loader&quot;);if(w[v]&amp;&amp;b.xhrWrappable){var g=t(10),x=t(11),E=t(8),P=t(6),O=t(13),R=t(7),T=t(14),L=t(9),j=t(&quot;ee&quot;),N=j.get(&quot;tracer&quot;);t(15),b.features.spa=!0;var S,M=0;j.on(u,r),j.on(p,r),j.on(d,o),j.on(h,o),j.buffer([u,d,&quot;xhr-done&quot;,&quot;xhr-resolved&quot;]),P.buffer([u]),O.buffer([&quot;setTimeout&quot;+s,&quot;clearTimeout&quot;+c,u]),T.buffer([u,&quot;new-xhr&quot;,&quot;send-xhr&quot;+c]),R.buffer([m+c,m+&quot;-done&quot;,m+f+c,m+f+s]),E.buffer([&quot;newURL&quot;]),g.buffer([u]),x.buffer([&quot;propagate&quot;,p,h,&quot;executor-err&quot;,&quot;resolve&quot;+c]),N.buffer([u,&quot;no-&quot;+u]),L.buffer([&quot;new-jsonp&quot;,&quot;cb-start&quot;,&quot;jsonp-error&quot;,&quot;jsonp-end&quot;]),a(T,&quot;send-xhr&quot;+c),a(j,&quot;xhr-resolved&quot;),a(j,&quot;xhr-done&quot;),a(R,m+c),a(R,m+&quot;-done&quot;),a(L,&quot;new-jsonp&quot;),a(L,&quot;jsonp-end&quot;),a(L,&quot;cb-start&quot;),E.on(&quot;pushState-end&quot;,i),E.on(&quot;replaceState-end&quot;,i),w[v](&quot;hashchange&quot;,i,!0),w[v](&quot;load&quot;,i,!0),w[v](&quot;popstate&quot;,function(){i(0,M>1)},!0)}},{}],5:[function(t,e,n){function r(t){}if(window.performance&amp;&amp;window.performance.timing&amp;&amp;window.performance.getEntriesByType){var o=t(&quot;ee&quot;),i=t(&quot;handle&quot;),a=t(13),c=t(12),s=&quot;learResourceTimings&quot;,f=&quot;addEventListener&quot;,u=&quot;resourcetimingbufferfull&quot;,d=&quot;bstResource&quot;,p=&quot;resource&quot;,h=&quot;-start&quot;,l=&quot;-end&quot;,m=&quot;fn&quot;+h,v=&quot;fn&quot;+l,w=&quot;bstTimer&quot;,y=&quot;pushState&quot;,b=t(&quot;loader&quot;);b.features.stn=!0,t(8);var g=NREUM.o.EV;o.on(m,function(t,e){var n=t[0];n instanceof g&amp;&amp;(this.bstStart=b.now())}),o.on(v,function(t,e){var n=t[0];n instanceof g&amp;&amp;i(&quot;bst&quot;,[n,e,this.bstStart,b.now()])}),a.on(m,function(t,e,n){this.bstStart=b.now(),this.bstType=n}),a.on(v,function(t,e){i(w,[e,this.bstStart,b.now(),this.bstType])}),c.on(m,function(){this.bstStart=b.now()}),c.on(v,function(t,e){i(w,[e,this.bstStart,b.now(),&quot;requestAnimationFrame&quot;])}),o.on(y+h,function(t){this.time=b.now(),this.startPath=location.pathname+location.hash}),o.on(y+l,function(t){i(&quot;bstHist&quot;,[location.pathname+location.hash,this.startPath,this.time])}),f in window.performance&amp;&amp;(window.performance[&quot;c&quot;+s]?window.performance[f](u,function(t){i(d,[window.performance.getEntriesByType(p)]),window.performance[&quot;c&quot;+s]()},!1):window.performance[f](&quot;webkit&quot;+u,function(t){i(d,[window.performance.getEntriesByType(p)]),window.performance[&quot;webkitC&quot;+s]()},!1)),document[f](&quot;scroll&quot;,r,{passive:!0}),document[f](&quot;keypress&quot;,r,!1),document[f](&quot;click&quot;,r,!1)}},{}],6:[function(t,e,n){function r(t){for(var e=t;e&amp;&amp;!e.hasOwnProperty(u);)e=Object.getPrototypeOf(e);e&amp;&amp;o(e)}function o(t){c.inPlace(t,[u,d],&quot;-&quot;,i)}function i(t,e){return t[1]}var a=t(&quot;ee&quot;).get(&quot;events&quot;),c=t(23)(a,!0),s=t(&quot;gos&quot;),f=XMLHttpRequest,u=&quot;addEventListener&quot;,d=&quot;removeEventListener&quot;;e.exports=a,&quot;getPrototypeOf&quot;in Object?(r(document),r(window),r(f.prototype)):f.prototype.hasOwnProperty(u)&amp;&amp;(o(window),o(f.prototype)),a.on(u+&quot;-start&quot;,function(t,e){var n=t[1],r=s(n,&quot;nr@wrapped&quot;,function(){function t(){if(&quot;function&quot;==typeof n.handleEvent)return n.handleEvent.apply(n,arguments)}var e={object:t,&quot;function&quot;:n}[typeof n];return e?c(e,&quot;fn-&quot;,null,e.name||&quot;anonymous&quot;):n});this.wrapped=t[1]=r}),a.on(d+&quot;-start&quot;,function(t){t[1]=this.wrapped||t[1]})},{}],7:[function(t,e,n){function r(t,e,n){var r=t[e];&quot;function&quot;==typeof r&amp;&amp;(t[e]=function(){var t=r.apply(this,arguments);return o.emit(n+&quot;start&quot;,arguments,t),t.then(function(e){return o.emit(n+&quot;end&quot;,[null,e],t),e},function(e){throw o.emit(n+&quot;end&quot;,[e],t),e})})}var o=t(&quot;ee&quot;).get(&quot;fetch&quot;),i=t(20);e.exports=o;var a=window,c=&quot;fetch-&quot;,s=c+&quot;body-&quot;,f=[&quot;arrayBuffer&quot;,&quot;blob&quot;,&quot;json&quot;,&quot;text&quot;,&quot;formData&quot;],u=a.Request,d=a.Response,p=a.fetch,h=&quot;prototype&quot;;u&amp;&amp;d&amp;&amp;p&amp;&amp;(i(f,function(t,e){r(u[h],e,s),r(d[h],e,s)}),r(a,&quot;fetch&quot;,c),o.on(c+&quot;end&quot;,function(t,e){var n=this;e?e.clone().arrayBuffer().then(function(t){n.rxSize=t.byteLength,o.emit(c+&quot;done&quot;,[null,e],n)}):o.emit(c+&quot;done&quot;,[t],n)}))},{}],8:[function(t,e,n){var r=t(&quot;ee&quot;).get(&quot;history&quot;),o=t(23)(r);e.exports=r,o.inPlace(window.history,[&quot;pushState&quot;,&quot;replaceState&quot;],&quot;-&quot;)},{}],9:[function(t,e,n){function r(t){function e(){s.emit(&quot;jsonp-end&quot;,[],p),t.removeEventListener(&quot;load&quot;,e,!1),t.removeEventListener(&quot;error&quot;,n,!1)}function n(){s.emit(&quot;jsonp-error&quot;,[],p),s.emit(&quot;jsonp-end&quot;,[],p),t.removeEventListener(&quot;load&quot;,e,!1),t.removeEventListener(&quot;error&quot;,n,!1)}var r=t&amp;&amp;&quot;string&quot;==typeof t.nodeName&amp;&amp;&quot;script&quot;===t.nodeName.toLowerCase();if(r){var o=&quot;function&quot;==typeof t.addEventListener;if(o){var a=i(t.src);if(a){var u=c(a),d=&quot;function&quot;==typeof u.parent[u.key];if(d){var p={};f.inPlace(u.parent,[u.key],&quot;cb-&quot;,p),t.addEventListener(&quot;load&quot;,e,!1),t.addEventListener(&quot;error&quot;,n,!1),s.emit(&quot;new-jsonp&quot;,[t.src],p)}}}}}function o(){return&quot;addEventListener&quot;in window}function i(t){var e=t.match(u);return e?e[1]:null}function a(t,e){var n=t.match(p),r=n[1],o=n[3];return o?a(o,e[r]):e[r]}function c(t){var e=t.match(d);return e&amp;&amp;e.length>=3?{key:e[2],parent:a(e[1],window)}:{key:t,parent:window}}var s=t(&quot;ee&quot;).get(&quot;jsonp&quot;),f=t(23)(s);if(e.exports=s,o()){var u=/[?&amp;](?:callback|cb)=([^&amp;#]+)/,d=/(.*)\.([^.]+)/,p=/^(\w+)(\.|$)(.*)$/,h=[&quot;appendChild&quot;,&quot;insertBefore&quot;,&quot;replaceChild&quot;];f.inPlace(HTMLElement.prototype,h,&quot;dom-&quot;),f.inPlace(HTMLHeadElement.prototype,h,&quot;dom-&quot;),f.inPlace(HTMLBodyElement.prototype,h,&quot;dom-&quot;),s.on(&quot;dom-start&quot;,function(t){r(t[0])})}},{}],10:[function(t,e,n){var r=t(&quot;ee&quot;).get(&quot;mutation&quot;),o=t(23)(r),i=NREUM.o.MO;e.exports=r,i&amp;&amp;(window.MutationObserver=function(t){return this instanceof i?new i(o(t,&quot;fn-&quot;)):i.apply(this,arguments)},MutationObserver.prototype=i.prototype)},{}],11:[function(t,e,n){function r(t){var e=a.context(),n=c(t,&quot;executor-&quot;,e),r=new f(n);return a.context(r).getCtx=function(){return e},a.emit(&quot;new-promise&quot;,[r,e],e),r}function o(t,e){return e}var i=t(23),a=t(&quot;ee&quot;).get(&quot;promise&quot;),c=i(a),s=t(20),f=NREUM.o.PR;e.exports=a,f&amp;&amp;(window.Promise=r,[&quot;all&quot;,&quot;race&quot;].forEach(function(t){var e=f[t];f[t]=function(n){function r(t){return function(){a.emit(&quot;propagate&quot;,[null,!o],i),o=o||!t}}var o=!1;s(n,function(e,n){Promise.resolve(n).then(r(&quot;all&quot;===t),r(!1))});var i=e.apply(f,arguments),c=f.resolve(i);return c}}),[&quot;resolve&quot;,&quot;reject&quot;].forEach(function(t){var e=f[t];f[t]=function(t){var n=e.apply(f,arguments);return t!==n&amp;&amp;a.emit(&quot;propagate&quot;,[t,!0],n),n}}),f.prototype[&quot;catch&quot;]=function(t){return this.then(null,t)},f.prototype=Object.create(f.prototype,{constructor:{value:r}}),s(Object.getOwnPropertyNames(f),function(t,e){try{r[e]=f[e]}catch(n){}}),a.on(&quot;executor-start&quot;,function(t){t[0]=c(t[0],&quot;resolve-&quot;,this),t[1]=c(t[1],&quot;resolve-&quot;,this)}),a.on(&quot;executor-err&quot;,function(t,e,n){t[1](n)}),c.inPlace(f.prototype,[&quot;then&quot;],&quot;then-&quot;,o),a.on(&quot;then-start&quot;,function(t,e){this.promise=e,t[0]=c(t[0],&quot;cb-&quot;,this),t[1]=c(t[1],&quot;cb-&quot;,this)}),a.on(&quot;then-end&quot;,function(t,e,n){this.nextPromise=n;var r=this.promise;a.emit(&quot;propagate&quot;,[r,!0],n)}),a.on(&quot;cb-end&quot;,function(t,e,n){a.emit(&quot;propagate&quot;,[n,!0],this.nextPromise)}),a.on(&quot;propagate&quot;,function(t,e,n){this.getCtx&amp;&amp;!e||(this.getCtx=function(){if(t instanceof Promise)var e=a.context(t);return e&amp;&amp;e.getCtx?e.getCtx():this})}),r.toString=function(){return&quot;&quot;+f})},{}],12:[function(t,e,n){var r=t(&quot;ee&quot;).get(&quot;raf&quot;),o=t(23)(r),i=&quot;equestAnimationFrame&quot;;e.exports=r,o.inPlace(window,[&quot;r&quot;+i,&quot;mozR&quot;+i,&quot;webkitR&quot;+i,&quot;msR&quot;+i],&quot;raf-&quot;),r.on(&quot;raf-start&quot;,function(t){t[0]=o(t[0],&quot;fn-&quot;)})},{}],13:[function(t,e,n){function r(t,e,n){t[0]=a(t[0],&quot;fn-&quot;,null,n)}function o(t,e,n){this.method=n,this.timerDuration=isNaN(t[1])?0:+t[1],t[0]=a(t[0],&quot;fn-&quot;,this,n)}var i=t(&quot;ee&quot;).get(&quot;timer&quot;),a=t(23)(i),c=&quot;setTimeout&quot;,s=&quot;setInterval&quot;,f=&quot;clearTimeout&quot;,u=&quot;-start&quot;,d=&quot;-&quot;;e.exports=i,a.inPlace(window,[c,&quot;setImmediate&quot;],c+d),a.inPlace(window,[s],s+d),a.inPlace(window,[f,&quot;clearImmediate&quot;],f+d),i.on(s+u,r),i.on(c+u,o)},{}],14:[function(t,e,n){function r(t,e){d.inPlace(e,[&quot;onreadystatechange&quot;],&quot;fn-&quot;,c)}function o(){var t=this,e=u.context(t);t.readyState>3&amp;&amp;!e.resolved&amp;&amp;(e.resolved=!0,u.emit(&quot;xhr-resolved&quot;,[],t)),d.inPlace(t,y,&quot;fn-&quot;,c)}function i(t){b.push(t),l&amp;&amp;(x?x.then(a):v?v(a):(E=-E,P.data=E))}function a(){for(var t=0;t&lt;b.length;t++)r([],b[t]);b.length&amp;&amp;(b=[])}function c(t,e){return e}function s(t,e){for(var n in t)e[n]=t[n];return e}t(6);var f=t(&quot;ee&quot;),u=f.get(&quot;xhr&quot;),d=t(23)(u),p=NREUM.o,h=p.XHR,l=p.MO,m=p.PR,v=p.SI,w=&quot;readystatechange&quot;,y=[&quot;onload&quot;,&quot;onerror&quot;,&quot;onabort&quot;,&quot;onloadstart&quot;,&quot;onloadend&quot;,&quot;onprogress&quot;,&quot;ontimeout&quot;],b=[];e.exports=u;var g=window.XMLHttpRequest=function(t){var e=new h(t);try{u.emit(&quot;new-xhr&quot;,[e],e),e.addEventListener(w,o,!1)}catch(n){try{u.emit(&quot;internal-error&quot;,[n])}catch(r){}}return e};if(s(h,g),g.prototype=h.prototype,d.inPlace(g.prototype,[&quot;open&quot;,&quot;send&quot;],&quot;-xhr-&quot;,c),u.on(&quot;send-xhr-start&quot;,function(t,e){r(t,e),i(e)}),u.on(&quot;open-xhr-start&quot;,r),l){var x=m&amp;&amp;m.resolve();if(!v&amp;&amp;!m){var E=1,P=document.createTextNode(E);new l(a).observe(P,{characterData:!0})}}else f.on(&quot;fn-end&quot;,function(t){t[0]&amp;&amp;t[0].type===w||a()})},{}],15:[function(t,e,n){function r(t){var e=this.params,n=this.metrics;if(!this.ended){this.ended=!0;for(var r=0;r&lt;d;r++)t.removeEventListener(u[r],this.listener,!1);if(!e.aborted){if(n.duration=a.now()-this.startTime,4===t.readyState){e.status=t.status;var i=o(t,this.lastSize);if(i&amp;&amp;(n.rxSize=i),this.sameOrigin){var s=t.getResponseHeader(&quot;X-NewRelic-App-Data&quot;);s&amp;&amp;(e.cat=s.split(&quot;, &quot;).pop())}}else e.status=0;n.cbTime=this.cbTime,f.emit(&quot;xhr-done&quot;,[t],t),c(&quot;xhr&quot;,[e,n,this.startTime])}}}function o(t,e){var n=t.responseType;if(&quot;json&quot;===n&amp;&amp;null!==e)return e;var r=&quot;arraybuffer&quot;===n||&quot;blob&quot;===n||&quot;json&quot;===n?t.response:t.responseText;return l(r)}function i(t,e){var n=s(e),r=t.params;r.host=n.hostname+&quot;:&quot;+n.port,r.pathname=n.pathname,t.sameOrigin=n.sameOrigin}var a=t(&quot;loader&quot;);if(a.xhrWrappable){var c=t(&quot;handle&quot;),s=t(16),f=t(&quot;ee&quot;),u=[&quot;load&quot;,&quot;error&quot;,&quot;abort&quot;,&quot;timeout&quot;],d=u.length,p=t(&quot;id&quot;),h=t(19),l=t(18),m=window.XMLHttpRequest;a.features.xhr=!0,t(14),f.on(&quot;new-xhr&quot;,function(t){var e=this;e.totalCbs=0,e.called=0,e.cbTime=0,e.end=r,e.ended=!1,e.xhrGuids={},e.lastSize=null,h&amp;&amp;(h>34||h&lt;10)||window.opera||t.addEventListener(&quot;progress&quot;,function(t){e.lastSize=t.loaded},!1)}),f.on(&quot;open-xhr-start&quot;,function(t){this.params={method:t[0]},i(this,t[1]),this.metrics={}}),f.on(&quot;open-xhr-end&quot;,function(t,e){&quot;loader_config&quot;in NREUM&amp;&amp;&quot;xpid&quot;in NREUM.loader_config&amp;&amp;this.sameOrigin&amp;&amp;e.setRequestHeader(&quot;X-NewRelic-ID&quot;,NREUM.loader_config.xpid)}),f.on(&quot;send-xhr-start&quot;,function(t,e){var n=this.metrics,r=t[0],o=this;if(n&amp;&amp;r){var i=l(r);i&amp;&amp;(n.txSize=i)}this.startTime=a.now(),this.listener=function(t){try{&quot;abort&quot;===t.type&amp;&amp;(o.params.aborted=!0),(&quot;load&quot;!==t.type||o.called===o.totalCbs&amp;&amp;(o.onloadCalled||&quot;function&quot;!=typeof e.onload))&amp;&amp;o.end(e)}catch(n){try{f.emit(&quot;internal-error&quot;,[n])}catch(r){}}};for(var c=0;c&lt;d;c++)e.addEventListener(u[c],this.listener,!1)}),f.on(&quot;xhr-cb-time&quot;,function(t,e,n){this.cbTime+=t,e?this.onloadCalled=!0:this.called+=1,this.called!==this.totalCbs||!this.onloadCalled&amp;&amp;&quot;function&quot;==typeof n.onload||this.end(n)}),f.on(&quot;xhr-load-added&quot;,function(t,e){var n=&quot;&quot;+p(t)+!!e;this.xhrGuids&amp;&amp;!this.xhrGuids[n]&amp;&amp;(this.xhrGuids[n]=!0,this.totalCbs+=1)}),f.on(&quot;xhr-load-removed&quot;,function(t,e){var n=&quot;&quot;+p(t)+!!e;this.xhrGuids&amp;&amp;this.xhrGuids[n]&amp;&amp;(delete this.xhrGuids[n],this.totalCbs-=1)}),f.on(&quot;addEventListener-end&quot;,function(t,e){e instanceof m&amp;&amp;&quot;load&quot;===t[0]&amp;&amp;f.emit(&quot;xhr-load-added&quot;,[t[1],t[2]],e)}),f.on(&quot;removeEventListener-end&quot;,function(t,e){e instanceof m&amp;&amp;&quot;load&quot;===t[0]&amp;&amp;f.emit(&quot;xhr-load-removed&quot;,[t[1],t[2]],e)}),f.on(&quot;fn-start&quot;,function(t,e,n){e instanceof m&amp;&amp;(&quot;onload&quot;===n&amp;&amp;(this.onload=!0),(&quot;load&quot;===(t[0]&amp;&amp;t[0].type)||this.onload)&amp;&amp;(this.xhrCbStart=a.now()))}),f.on(&quot;fn-end&quot;,function(t,e){this.xhrCbStart&amp;&amp;f.emit(&quot;xhr-cb-time&quot;,[a.now()-this.xhrCbStart,this.onload,e],e)})}},{}],16:[function(t,e,n){e.exports=function(t){var e=document.createElement(&quot;a&quot;),n=window.location,r={};e.href=t,r.port=e.port;var o=e.href.split(&quot;://&quot;);!r.port&amp;&amp;o[1]&amp;&amp;(r.port=o[1].split(&quot;/&quot;)[0].split(&quot;@&quot;).pop().split(&quot;:&quot;)[1]),r.port&amp;&amp;&quot;0&quot;!==r.port||(r.port=&quot;https&quot;===o[0]?&quot;443&quot;:&quot;80&quot;),r.hostname=e.hostname||n.hostname,r.pathname=e.pathname,r.protocol=o[0],&quot;/&quot;!==r.pathname.charAt(0)&amp;&amp;(r.pathname=&quot;/&quot;+r.pathname);var i=!e.protocol||&quot;:&quot;===e.protocol||e.protocol===n.protocol,a=e.hostname===document.domain&amp;&amp;e.port===n.port;return r.sameOrigin=i&amp;&amp;(!e.hostname||a),r}},{}],17:[function(t,e,n){function r(){}function o(t,e,n){return function(){return i(t,[f.now()].concat(c(arguments)),e?null:this,n),e?void 0:this}}var i=t(&quot;handle&quot;),a=t(20),c=t(21),s=t(&quot;ee&quot;).get(&quot;tracer&quot;),f=t(&quot;loader&quot;),u=NREUM;&quot;undefined&quot;==typeof window.newrelic&amp;&amp;(newrelic=u);var d=[&quot;setPageViewName&quot;,&quot;setCustomAttribute&quot;,&quot;setErrorHandler&quot;,&quot;finished&quot;,&quot;addToTrace&quot;,&quot;inlineHit&quot;,&quot;addRelease&quot;],p=&quot;api-&quot;,h=p+&quot;ixn-&quot;;a(d,function(t,e){u[e]=o(p+e,!0,&quot;api&quot;)}),u.addPageAction=o(p+&quot;addPageAction&quot;,!0),u.setCurrentRouteName=o(p+&quot;routeName&quot;,!0),e.exports=newrelic,u.interaction=function(){return(new r).get()};var l=r.prototype={createTracer:function(t,e){var n={},r=this,o=&quot;function&quot;==typeof e;return i(h+&quot;tracer&quot;,[f.now(),t,n],r),function(){if(s.emit((o?&quot;&quot;:&quot;no-&quot;)+&quot;fn-start&quot;,[f.now(),r,o],n),o)try{return e.apply(this,arguments)}catch(t){throw s.emit(&quot;fn-err&quot;,[arguments,this,t],n),t}finally{s.emit(&quot;fn-end&quot;,[f.now()],n)}}}};a(&quot;setName,setAttribute,save,ignore,onEnd,getContext,end,get&quot;.split(&quot;,&quot;),function(t,e){l[e]=o(h+e)}),newrelic.noticeError=function(t){&quot;string&quot;==typeof t&amp;&amp;(t=new Error(t)),i(&quot;err&quot;,[t,f.now()])}},{}],18:[function(t,e,n){e.exports=function(t){if(&quot;string&quot;==typeof t&amp;&amp;t.length)return t.length;if(&quot;object&quot;==typeof t){if(&quot;undefined&quot;!=typeof ArrayBuffer&amp;&amp;t instanceof ArrayBuffer&amp;&amp;t.byteLength)return t.byteLength;if(&quot;undefined&quot;!=typeof Blob&amp;&amp;t instanceof Blob&amp;&amp;t.size)return t.size;if(!(&quot;undefined&quot;!=typeof FormData&amp;&amp;t instanceof FormData))try{return JSON.stringify(t).length}catch(e){return}}}},{}],19:[function(t,e,n){var r=0,o=navigator.userAgent.match(/Firefox[\/\s](\d+\.\d+)/);o&amp;&amp;(r=+o[1]),e.exports=r},{}],20:[function(t,e,n){function r(t,e){var n=[],r=&quot;&quot;,i=0;for(r in t)o.call(t,r)&amp;&amp;(n[i]=e(r,t[r]),i+=1);return n}var o=Object.prototype.hasOwnProperty;e.exports=r},{}],21:[function(t,e,n){function r(t,e,n){e||(e=0),&quot;undefined&quot;==typeof n&amp;&amp;(n=t?t.length:0);for(var r=-1,o=n-e||0,i=Array(o&lt;0?0:o);++r&lt;o;)i[r]=t[e+r];return i}e.exports=r},{}],22:[function(t,e,n){e.exports={exists:&quot;undefined&quot;!=typeof window.performance&amp;&amp;window.performance.timing&amp;&amp;&quot;undefined&quot;!=typeof window.performance.timing.navigationStart}},{}],23:[function(t,e,n){function r(t){return!(t&amp;&amp;t instanceof Function&amp;&amp;t.apply&amp;&amp;!t[a])}var o=t(&quot;ee&quot;),i=t(21),a=&quot;nr@original&quot;,c=Object.prototype.hasOwnProperty,s=!1;e.exports=function(t,e){function n(t,e,n,o){function nrWrapper(){var r,a,c,s;try{a=this,r=i(arguments),c=&quot;function&quot;==typeof n?n(r,a):n||{}}catch(f){p([f,&quot;&quot;,[r,a,o],c])}u(e+&quot;start&quot;,[r,a,o],c);try{return s=t.apply(a,r)}catch(d){throw u(e+&quot;err&quot;,[r,a,d],c),d}finally{u(e+&quot;end&quot;,[r,a,s],c)}}return r(t)?t:(e||(e=&quot;&quot;),nrWrapper[a]=t,d(t,nrWrapper),nrWrapper)}function f(t,e,o,i){o||(o=&quot;&quot;);var a,c,s,f=&quot;-&quot;===o.charAt(0);for(s=0;s&lt;e.length;s++)c=e[s],a=t[c],r(a)||(t[c]=n(a,f?c+o:o,i,c))}function u(n,r,o){if(!s||e){var i=s;s=!0;try{t.emit(n,r,o,e)}catch(a){p([a,n,r,o])}s=i}}function d(t,e){if(Object.defineProperty&amp;&amp;Object.keys)try{var n=Object.keys(t);return n.forEach(function(n){Object.defineProperty(e,n,{get:function(){return t[n]},set:function(e){return t[n]=e,e}})}),e}catch(r){p([r])}for(var o in t)c.call(t,o)&amp;&amp;(e[o]=t[o]);return e}function p(e){try{t.emit(&quot;internal-error&quot;,e)}catch(n){}}return t||(t=o),n.inPlace=f,n.flag=a,n}},{}],ee:[function(t,e,n){function r(){}function o(t){function e(t){return t&amp;&amp;t instanceof r?t:t?s(t,c,i):i()}function n(n,r,o,i){if(!p.aborted||i){t&amp;&amp;t(n,r,o);for(var a=e(o),c=l(n),s=c.length,f=0;f&lt;s;f++)c[f].apply(a,r);var d=u[y[n]];return d&amp;&amp;d.push([b,n,r,a]),a}}function h(t,e){w[t]=l(t).concat(e)}function l(t){return w[t]||[]}function m(t){return d[t]=d[t]||o(n)}function v(t,e){f(t,function(t,n){e=e||&quot;feature&quot;,y[n]=e,e in u||(u[e]=[])})}var w={},y={},b={on:h,emit:n,get:m,listeners:l,context:e,buffer:v,abort:a,aborted:!1};return b}function i(){return new r}function a(){(u.api||u.feature)&amp;&amp;(p.aborted=!0,u=p.backlog={})}var c=&quot;nr@context&quot;,s=t(&quot;gos&quot;),f=t(20),u={},d={},p=e.exports=o();p.backlog=u},{}],gos:[function(t,e,n){function r(t,e,n){if(o.call(t,e))return t[e];var r=n();if(Object.defineProperty&amp;&amp;Object.keys)try{return Object.defineProperty(t,e,{value:r,writable:!0,enumerable:!1}),r}catch(i){}return t[e]=r,r}var o=Object.prototype.hasOwnProperty;e.exports=r},{}],handle:[function(t,e,n){function r(t,e,n,r){o.buffer([t],r),o.emit(t,e,n)}var o=t(&quot;ee&quot;).get(&quot;handle&quot;);e.exports=r,r.ee=o},{}],id:[function(t,e,n){function r(t){var e=typeof t;return!t||&quot;object&quot;!==e&amp;&amp;&quot;function&quot;!==e?-1:t===window?0:a(t,i,function(){return o++})}var o=1,i=&quot;nr@id&quot;,a=t(&quot;gos&quot;);e.exports=r},{}],loader:[function(t,e,n){function r(){if(!x++){var t=g.info=NREUM.info,e=p.getElementsByTagName(&quot;script&quot;)[0];if(setTimeout(u.abort,3e4),!(t&amp;&amp;t.licenseKey&amp;&amp;t.applicationID&amp;&amp;e))return u.abort();f(y,function(e,n){t[e]||(t[e]=n)}),s(&quot;mark&quot;,[&quot;onload&quot;,a()+g.offset],null,&quot;api&quot;);var n=p.createElement(&quot;script&quot;);n.src=&quot;https://&quot;+t.agent,e.parentNode.insertBefore(n,e)}}function o(){&quot;complete&quot;===p.readyState&amp;&amp;i()}function i(){s(&quot;mark&quot;,[&quot;domContent&quot;,a()+g.offset],null,&quot;api&quot;)}function a(){return E.exists&amp;&amp;performance.now?Math.round(performance.now()):(c=Math.max((new Date).getTime(),c))-g.offset}var c=(new Date).getTime(),s=t(&quot;handle&quot;),f=t(20),u=t(&quot;ee&quot;),d=window,p=d.document,h=&quot;addEventListener&quot;,l=&quot;attachEvent&quot;,m=d.XMLHttpRequest,v=m&amp;&amp;m.prototype;NREUM.o={ST:setTimeout,SI:d.setImmediate,CT:clearTimeout,XHR:m,REQ:d.Request,EV:d.Event,PR:d.Promise,MO:d.MutationObserver};var w=&quot;&quot;+location,y={beacon:&quot;bam.nr-data.net&quot;,errorBeacon:&quot;bam.nr-data.net&quot;,agent:&quot;js-agent.newrelic.com/nr-spa-1071.min.js&quot;},b=m&amp;&amp;v&amp;&amp;v[h]&amp;&amp;!/CriOS/.test(navigator.userAgent),g=e.exports={offset:c,now:a,origin:w,features:{},xhrWrappable:b};t(17),p[h]?(p[h](&quot;DOMContentLoaded&quot;,i,!1),d[h](&quot;load&quot;,r,!1)):(p[l](&quot;onreadystatechange&quot;,o),d[l](&quot;onload&quot;,r)),s(&quot;mark&quot;,[&quot;firstbyte&quot;,c],null,&quot;api&quot;);var x=0,E=t(22)},{}]},{},[&quot;loader&quot;,2,15,5,3,4]);



	

	

	

	

	

	

	

	

	

	

	

	

	

	





#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-elementInfoDiv {color: lightblue; padding: 5px}

	
		















    


    var contextPath = ' , &quot;'&quot; , '/DCM_UI' , &quot;'&quot; , ';
    var servletPath = ' , &quot;'&quot; , '/create-location-group' , &quot;'&quot; , ';
    var userId = ' , &quot;'&quot; , 'p165114' , &quot;'&quot; , ';
    var userRole = ' , &quot;'&quot; , 'ADMIN' , &quot;'&quot; , ';
    var userName = ' , &quot;'&quot; , 'Peters,Carrol' , &quot;'&quot; , ';
    var sessionTimeout = ' , &quot;'&quot; , '1800' , &quot;'&quot; , ';

    function formSubmit() {
        document.getElementById(&quot;logoutForm&quot;).submit();
    }


       Warning! Due to inactivity, your session will expire in 00:00:00. To extend your session another 30 minute(s), please press the Extend button.
    Extend


   Warning! Due to inactivity, your session has expired. Please save any data you may have entered before refreshing the page.


    
        
            
        
        Cost and Deals
    
    
        
            
                
                    
                    
                    
                
                Logged in as
                    
                    
                    
                    Peters,Carrol
                    
                    
                
                Sign Out
            
        
        
            
                Toggle navigation
                
                
                
            
        
        
            
                
                    Home
                    
                        
                            Switch To...
                            Announcements
                        
                    
                    
                
                
                    Offer
                    
                        
                            Create
                        
                        Deal Builder
                        Review
                        
                            Accept/Reject
                        
                        Search
                        
                            Maintain
                            Mass Upload
                        
                    
                
                Deals
                    
                        Search
                        Maintain
                        SODL
                            
                                Search &amp; Maintenance
                                Details
                                Transactions
                            
                        
                        Browse Deals and Costs
                    
                
                Cost
                    
                        Maintain
                        Browse Deals and Costs
                    
                
                Location Group
                    
                        
                            Create
                        
                        Search
                        
                            Maintain
                        
                    
                
                Items
                    
                        Cost Link Maintenance
                    
                
                
                    People
                        
                            User Preferences
                            Forward Alert and Share Authority
                        
                    
                
                
                
                Help
                
            
        
    
    
        
            
            
            
            
        
        
            
                
                    Help
                    
                
            

            
                
                    
                
                
                    
                
            
        


        
        
            
            	
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                    
                    HEB Enterprise Portal
                    Cost and Deals
                    Location Group
                    Create
                
                
                
                
                
                
                
                
                
            
        
    

            
            	
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                    
                    HEB Enterprise Portal
                    Cost and Deals
                    Location Group
                    Create
                
                
                
                
                
                
                
                
                
            
        

    
        
            
                User Setting
                ×
            
            
                
                    General
                    Custom Settings
                
                
                    
                        
                        
                            Dashboard
                            
                                 Show Information Alerts
                            
                            
                                 Activate &quot;Switch to&quot; User
                            
                            
                                Switch to User Id 
                                
                                    
                                
                                
                            
                        
                        
                            Available Partners
                            
                        
                        
                    
                    
                        
                        
                            Screen
                            
                                 Application 
                            
                            
                                 Screen Name 
                            
                            
                                 Tab Name 
                            
                        
                        
                            Misc.
                            
                                 Default Start Date to 
                                
                                 Days Prior to Today 
                            
                        
                        
                            Available Columns
                            
                        
                        
                    
                
                
                    OK
                    Cancel
                    Reset to Defaults
                
            
        
    


    var highlightMenuItem = ' , &quot;'&quot; , 'basket-create' , &quot;'&quot; , ';
    document.addEventListener(&quot;DOMContentLoaded&quot;, function(event) {
        if (highlightMenuItem !== ' , &quot;'&quot; , '' , &quot;'&quot; , ') {
            $(' , &quot;'&quot; , '*[data-highlight=&quot;' , &quot;'&quot; , ' + highlightMenuItem + ' , &quot;'&quot; , '&quot;]' , &quot;'&quot; , ').addClass(' , &quot;'&quot; , 'active' , &quot;'&quot; , ');
            $(' , &quot;'&quot; , '*[data-highlight=&quot;' , &quot;'&quot; , ' + highlightMenuItem + ' , &quot;'&quot; , '&quot;]' , &quot;'&quot; , ').closest(' , &quot;'&quot; , 'li.dropdown' , &quot;'&quot; , ').addClass(' , &quot;'&quot; , 'active' , &quot;'&quot; , ');
        }
    });


		
			
	
		CREATE LOCATION GROUP
		
			
				
			
			
				
			
			
				
			
			
				
			
			
				
			
		
	
	
		
		
		
			
				
					
						The following errors were encountered:
						
					
					
						
					
				
				
			
		
		
		
			
				Location Group # or Name
			
			
				
				
					
						Name:
						add new
					
					
						Owner:
						p165114 Peters,Carrol
					
					
						Domain:
						HEB - HEB ONLY PARTNERS
					
					
						Type:
						LOCATION
					
				

			
			
				
					
						Details for Location Group #
						
					
				
				

					
						
							
								Note: Required fields are marked with an asterisk (*)
							
							
								
								
									
										Group Name
									
									
										
										
											*
										
									
								
								
								
								
									
										Owner
									
									
										
										
											*
										
									
									
										Peters,Carrol
									
								
								
								
								
									
										Domain
									
									
										
											HEB - HEB ONLY PARTNERS - Select -HEB - HEB ONLY PARTNERSVEND - BOTH HEB AND VENDORS- Select -HEB - HEB ONLY PARTNERSVEND - BOTH HEB AND VENDORS
											
										
										
											*
										
									
								
								
								
								
									
										Type
									
									
										STORE - LOCATION
										
										
									
								

								
									
										Public
									
									
										
									
								

								
									
										Make Location Group usable by
									
									
										
									
									
										
									
								

							
						
						
							
								
									 
								
								
									
										Abbreviation
									
									
										
										
											*
										
									
								
								
								
									
										Description
									
									
										
											
										
									
								
								
								
									
										Level
									
									
										
											USER - USER DEFINED CGRP - COST GROUPCORP - CORPORATE DERIVEDUSER - USER DEFINEDCGRP - COST GROUPCORP - CORPORATE DERIVEDUSER - USER DEFINED
											
										
									
								
								
								
									
										No of Elements
									
									
										0
									
								
								
									
										Active
									
									
										
									
								
							
						

						
							
								
									 
								
								
									
										Public Comments
									
									
										
									
								
								
									
										Private Comments
									
									
										
									
								
							
							
								
									
										Created By
									
									
										
									
								
							
							
								
									
										Updated By
									
									
										
									
								
							
						
					
				
			
		
		
			
				
					
				
			
			
				
					
						
							Select
						
						
						
							
								
								
								
							
							
							
								
							
						
						
							All Stores
								
									
								00000 - 00001 - BR01 ELIZABETH00006 - STEPHENVILLE00009 - DONNA00013 - RIO GRANDE CITY-HWY83/S BRIDGE00014 - KYLE00016 - BURLESON - JOHN JONES/WILSHIRE00017 - ANGLETON00019 - LYTLE    FM 2790/IH3500020 - H53 FM 290/BARKER CYPRESS
							
						
						
							Page 1 of 39
							Total items 386
						
						
							12345Next >Last >>
						
					
				
				
					
						
					
					
						
					
					
						
					
				
				
					
						
							Add to Location Group
						
						
						
							
								
								
							
							
							
								
							
						
						
							
								
									
								
							
						
					
				
			
		
		

		
			
				
					
						
							Help
							
						
					

				
			
		

	
		var basketIdFromBasketSearch = &quot;0&quot;; //int
		var isCopyFromBasketSearch = &quot;&quot;;
	

		
	
	
	

window.NREUM||(NREUM={});NREUM.info={&quot;errorBeacon&quot;:&quot;bam.nr-data.net&quot;,&quot;licenseKey&quot;:&quot;107f4f8ae4&quot;,&quot;agent&quot;:&quot;&quot;,&quot;beacon&quot;:&quot;bam.nr-data.net&quot;,&quot;applicationTime&quot;:6,&quot;applicationID&quot;:&quot;108906967&quot;,&quot;transactionName&quot;:&quot;NVNSN0dTXkRTBkFZDQwZYxNHW15QcQpbRBANWlwGRx1TRVcEQVVPDllTAkFbX1kfAkdfFxIWGCRwZhk=&quot;,&quot;queueTime&quot;:0}


	

	

	

	

	

	

	

	

	

	

	

	

	

	

	

	

	

	

	

	

	

	



	

	

	


/html[1]'))]</value>
      </entry>
   </selectorCollection>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>html</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>ng-app</name>
      <type>Main</type>
      <value>dcmApp</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>






Create Location Group - Cost and Deals


	
(window.NREUM||(NREUM={})).loader_config={xpid:&quot;VQAPUFZSDBABVFlbBwcOUFM=&quot;};window.NREUM||(NREUM={}),__nr_require=function(t,e,n){function r(n){if(!e[n]){var o=e[n]={exports:{}};t[n][0].call(o.exports,function(e){var o=t[n][1][e];return r(o||e)},o,o.exports)}return e[n].exports}if(&quot;function&quot;==typeof __nr_require)return __nr_require;for(var o=0;o&lt;n.length;o++)r(n[o]);return r}({1:[function(t,e,n){function r(t){try{c.console&amp;&amp;console.log(t)}catch(e){}}var o,i=t(&quot;ee&quot;),a=t(20),c={};try{o=localStorage.getItem(&quot;__nr_flags&quot;).split(&quot;,&quot;),console&amp;&amp;&quot;function&quot;==typeof console.log&amp;&amp;(c.console=!0,o.indexOf(&quot;dev&quot;)!==-1&amp;&amp;(c.dev=!0),o.indexOf(&quot;nr_dev&quot;)!==-1&amp;&amp;(c.nrDev=!0))}catch(s){}c.nrDev&amp;&amp;i.on(&quot;internal-error&quot;,function(t){r(t.stack)}),c.dev&amp;&amp;i.on(&quot;fn-err&quot;,function(t,e,n){r(n.stack)}),c.dev&amp;&amp;(r(&quot;NR AGENT IN DEVELOPMENT MODE&quot;),r(&quot;flags: &quot;+a(c,function(t,e){return t}).join(&quot;, &quot;)))},{}],2:[function(t,e,n){function r(t,e,n,r,c){try{h?h-=1:o(c||new UncaughtException(t,e,n),!0)}catch(f){try{i(&quot;ierr&quot;,[f,s.now(),!0])}catch(d){}}return&quot;function&quot;==typeof u&amp;&amp;u.apply(this,a(arguments))}function UncaughtException(t,e,n){this.message=t||&quot;Uncaught error with no additional information&quot;,this.sourceURL=e,this.line=n}function o(t,e){var n=e?null:s.now();i(&quot;err&quot;,[t,n])}var i=t(&quot;handle&quot;),a=t(21),c=t(&quot;ee&quot;),s=t(&quot;loader&quot;),f=t(&quot;gos&quot;),u=window.onerror,d=!1,p=&quot;nr@seenError&quot;,h=0;s.features.err=!0,t(1),window.onerror=r;try{throw new Error}catch(l){&quot;stack&quot;in l&amp;&amp;(t(13),t(12),&quot;addEventListener&quot;in window&amp;&amp;t(6),s.xhrWrappable&amp;&amp;t(14),d=!0)}c.on(&quot;fn-start&quot;,function(t,e,n){d&amp;&amp;(h+=1)}),c.on(&quot;fn-err&quot;,function(t,e,n){d&amp;&amp;!n[p]&amp;&amp;(f(n,p,function(){return!0}),this.thrown=!0,o(n))}),c.on(&quot;fn-end&quot;,function(){d&amp;&amp;!this.thrown&amp;&amp;h>0&amp;&amp;(h-=1)}),c.on(&quot;internal-error&quot;,function(t){i(&quot;ierr&quot;,[t,s.now(),!0])})},{}],3:[function(t,e,n){t(&quot;loader&quot;).features.ins=!0},{}],4:[function(t,e,n){function r(){M++,S=y.hash,this[u]=b.now()}function o(){M--,y.hash!==S&amp;&amp;i(0,!0);var t=b.now();this[l]=~~this[l]+t-this[u],this[d]=t}function i(t,e){E.emit(&quot;newURL&quot;,[&quot;&quot;+y,e])}function a(t,e){t.on(e,function(){this[e]=b.now()})}var c=&quot;-start&quot;,s=&quot;-end&quot;,f=&quot;-body&quot;,u=&quot;fn&quot;+c,d=&quot;fn&quot;+s,p=&quot;cb&quot;+c,h=&quot;cb&quot;+s,l=&quot;jsTime&quot;,m=&quot;fetch&quot;,v=&quot;addEventListener&quot;,w=window,y=w.location,b=t(&quot;loader&quot;);if(w[v]&amp;&amp;b.xhrWrappable){var g=t(10),x=t(11),E=t(8),P=t(6),O=t(13),R=t(7),T=t(14),L=t(9),j=t(&quot;ee&quot;),N=j.get(&quot;tracer&quot;);t(15),b.features.spa=!0;var S,M=0;j.on(u,r),j.on(p,r),j.on(d,o),j.on(h,o),j.buffer([u,d,&quot;xhr-done&quot;,&quot;xhr-resolved&quot;]),P.buffer([u]),O.buffer([&quot;setTimeout&quot;+s,&quot;clearTimeout&quot;+c,u]),T.buffer([u,&quot;new-xhr&quot;,&quot;send-xhr&quot;+c]),R.buffer([m+c,m+&quot;-done&quot;,m+f+c,m+f+s]),E.buffer([&quot;newURL&quot;]),g.buffer([u]),x.buffer([&quot;propagate&quot;,p,h,&quot;executor-err&quot;,&quot;resolve&quot;+c]),N.buffer([u,&quot;no-&quot;+u]),L.buffer([&quot;new-jsonp&quot;,&quot;cb-start&quot;,&quot;jsonp-error&quot;,&quot;jsonp-end&quot;]),a(T,&quot;send-xhr&quot;+c),a(j,&quot;xhr-resolved&quot;),a(j,&quot;xhr-done&quot;),a(R,m+c),a(R,m+&quot;-done&quot;),a(L,&quot;new-jsonp&quot;),a(L,&quot;jsonp-end&quot;),a(L,&quot;cb-start&quot;),E.on(&quot;pushState-end&quot;,i),E.on(&quot;replaceState-end&quot;,i),w[v](&quot;hashchange&quot;,i,!0),w[v](&quot;load&quot;,i,!0),w[v](&quot;popstate&quot;,function(){i(0,M>1)},!0)}},{}],5:[function(t,e,n){function r(t){}if(window.performance&amp;&amp;window.performance.timing&amp;&amp;window.performance.getEntriesByType){var o=t(&quot;ee&quot;),i=t(&quot;handle&quot;),a=t(13),c=t(12),s=&quot;learResourceTimings&quot;,f=&quot;addEventListener&quot;,u=&quot;resourcetimingbufferfull&quot;,d=&quot;bstResource&quot;,p=&quot;resource&quot;,h=&quot;-start&quot;,l=&quot;-end&quot;,m=&quot;fn&quot;+h,v=&quot;fn&quot;+l,w=&quot;bstTimer&quot;,y=&quot;pushState&quot;,b=t(&quot;loader&quot;);b.features.stn=!0,t(8);var g=NREUM.o.EV;o.on(m,function(t,e){var n=t[0];n instanceof g&amp;&amp;(this.bstStart=b.now())}),o.on(v,function(t,e){var n=t[0];n instanceof g&amp;&amp;i(&quot;bst&quot;,[n,e,this.bstStart,b.now()])}),a.on(m,function(t,e,n){this.bstStart=b.now(),this.bstType=n}),a.on(v,function(t,e){i(w,[e,this.bstStart,b.now(),this.bstType])}),c.on(m,function(){this.bstStart=b.now()}),c.on(v,function(t,e){i(w,[e,this.bstStart,b.now(),&quot;requestAnimationFrame&quot;])}),o.on(y+h,function(t){this.time=b.now(),this.startPath=location.pathname+location.hash}),o.on(y+l,function(t){i(&quot;bstHist&quot;,[location.pathname+location.hash,this.startPath,this.time])}),f in window.performance&amp;&amp;(window.performance[&quot;c&quot;+s]?window.performance[f](u,function(t){i(d,[window.performance.getEntriesByType(p)]),window.performance[&quot;c&quot;+s]()},!1):window.performance[f](&quot;webkit&quot;+u,function(t){i(d,[window.performance.getEntriesByType(p)]),window.performance[&quot;webkitC&quot;+s]()},!1)),document[f](&quot;scroll&quot;,r,{passive:!0}),document[f](&quot;keypress&quot;,r,!1),document[f](&quot;click&quot;,r,!1)}},{}],6:[function(t,e,n){function r(t){for(var e=t;e&amp;&amp;!e.hasOwnProperty(u);)e=Object.getPrototypeOf(e);e&amp;&amp;o(e)}function o(t){c.inPlace(t,[u,d],&quot;-&quot;,i)}function i(t,e){return t[1]}var a=t(&quot;ee&quot;).get(&quot;events&quot;),c=t(23)(a,!0),s=t(&quot;gos&quot;),f=XMLHttpRequest,u=&quot;addEventListener&quot;,d=&quot;removeEventListener&quot;;e.exports=a,&quot;getPrototypeOf&quot;in Object?(r(document),r(window),r(f.prototype)):f.prototype.hasOwnProperty(u)&amp;&amp;(o(window),o(f.prototype)),a.on(u+&quot;-start&quot;,function(t,e){var n=t[1],r=s(n,&quot;nr@wrapped&quot;,function(){function t(){if(&quot;function&quot;==typeof n.handleEvent)return n.handleEvent.apply(n,arguments)}var e={object:t,&quot;function&quot;:n}[typeof n];return e?c(e,&quot;fn-&quot;,null,e.name||&quot;anonymous&quot;):n});this.wrapped=t[1]=r}),a.on(d+&quot;-start&quot;,function(t){t[1]=this.wrapped||t[1]})},{}],7:[function(t,e,n){function r(t,e,n){var r=t[e];&quot;function&quot;==typeof r&amp;&amp;(t[e]=function(){var t=r.apply(this,arguments);return o.emit(n+&quot;start&quot;,arguments,t),t.then(function(e){return o.emit(n+&quot;end&quot;,[null,e],t),e},function(e){throw o.emit(n+&quot;end&quot;,[e],t),e})})}var o=t(&quot;ee&quot;).get(&quot;fetch&quot;),i=t(20);e.exports=o;var a=window,c=&quot;fetch-&quot;,s=c+&quot;body-&quot;,f=[&quot;arrayBuffer&quot;,&quot;blob&quot;,&quot;json&quot;,&quot;text&quot;,&quot;formData&quot;],u=a.Request,d=a.Response,p=a.fetch,h=&quot;prototype&quot;;u&amp;&amp;d&amp;&amp;p&amp;&amp;(i(f,function(t,e){r(u[h],e,s),r(d[h],e,s)}),r(a,&quot;fetch&quot;,c),o.on(c+&quot;end&quot;,function(t,e){var n=this;e?e.clone().arrayBuffer().then(function(t){n.rxSize=t.byteLength,o.emit(c+&quot;done&quot;,[null,e],n)}):o.emit(c+&quot;done&quot;,[t],n)}))},{}],8:[function(t,e,n){var r=t(&quot;ee&quot;).get(&quot;history&quot;),o=t(23)(r);e.exports=r,o.inPlace(window.history,[&quot;pushState&quot;,&quot;replaceState&quot;],&quot;-&quot;)},{}],9:[function(t,e,n){function r(t){function e(){s.emit(&quot;jsonp-end&quot;,[],p),t.removeEventListener(&quot;load&quot;,e,!1),t.removeEventListener(&quot;error&quot;,n,!1)}function n(){s.emit(&quot;jsonp-error&quot;,[],p),s.emit(&quot;jsonp-end&quot;,[],p),t.removeEventListener(&quot;load&quot;,e,!1),t.removeEventListener(&quot;error&quot;,n,!1)}var r=t&amp;&amp;&quot;string&quot;==typeof t.nodeName&amp;&amp;&quot;script&quot;===t.nodeName.toLowerCase();if(r){var o=&quot;function&quot;==typeof t.addEventListener;if(o){var a=i(t.src);if(a){var u=c(a),d=&quot;function&quot;==typeof u.parent[u.key];if(d){var p={};f.inPlace(u.parent,[u.key],&quot;cb-&quot;,p),t.addEventListener(&quot;load&quot;,e,!1),t.addEventListener(&quot;error&quot;,n,!1),s.emit(&quot;new-jsonp&quot;,[t.src],p)}}}}}function o(){return&quot;addEventListener&quot;in window}function i(t){var e=t.match(u);return e?e[1]:null}function a(t,e){var n=t.match(p),r=n[1],o=n[3];return o?a(o,e[r]):e[r]}function c(t){var e=t.match(d);return e&amp;&amp;e.length>=3?{key:e[2],parent:a(e[1],window)}:{key:t,parent:window}}var s=t(&quot;ee&quot;).get(&quot;jsonp&quot;),f=t(23)(s);if(e.exports=s,o()){var u=/[?&amp;](?:callback|cb)=([^&amp;#]+)/,d=/(.*)\.([^.]+)/,p=/^(\w+)(\.|$)(.*)$/,h=[&quot;appendChild&quot;,&quot;insertBefore&quot;,&quot;replaceChild&quot;];f.inPlace(HTMLElement.prototype,h,&quot;dom-&quot;),f.inPlace(HTMLHeadElement.prototype,h,&quot;dom-&quot;),f.inPlace(HTMLBodyElement.prototype,h,&quot;dom-&quot;),s.on(&quot;dom-start&quot;,function(t){r(t[0])})}},{}],10:[function(t,e,n){var r=t(&quot;ee&quot;).get(&quot;mutation&quot;),o=t(23)(r),i=NREUM.o.MO;e.exports=r,i&amp;&amp;(window.MutationObserver=function(t){return this instanceof i?new i(o(t,&quot;fn-&quot;)):i.apply(this,arguments)},MutationObserver.prototype=i.prototype)},{}],11:[function(t,e,n){function r(t){var e=a.context(),n=c(t,&quot;executor-&quot;,e),r=new f(n);return a.context(r).getCtx=function(){return e},a.emit(&quot;new-promise&quot;,[r,e],e),r}function o(t,e){return e}var i=t(23),a=t(&quot;ee&quot;).get(&quot;promise&quot;),c=i(a),s=t(20),f=NREUM.o.PR;e.exports=a,f&amp;&amp;(window.Promise=r,[&quot;all&quot;,&quot;race&quot;].forEach(function(t){var e=f[t];f[t]=function(n){function r(t){return function(){a.emit(&quot;propagate&quot;,[null,!o],i),o=o||!t}}var o=!1;s(n,function(e,n){Promise.resolve(n).then(r(&quot;all&quot;===t),r(!1))});var i=e.apply(f,arguments),c=f.resolve(i);return c}}),[&quot;resolve&quot;,&quot;reject&quot;].forEach(function(t){var e=f[t];f[t]=function(t){var n=e.apply(f,arguments);return t!==n&amp;&amp;a.emit(&quot;propagate&quot;,[t,!0],n),n}}),f.prototype[&quot;catch&quot;]=function(t){return this.then(null,t)},f.prototype=Object.create(f.prototype,{constructor:{value:r}}),s(Object.getOwnPropertyNames(f),function(t,e){try{r[e]=f[e]}catch(n){}}),a.on(&quot;executor-start&quot;,function(t){t[0]=c(t[0],&quot;resolve-&quot;,this),t[1]=c(t[1],&quot;resolve-&quot;,this)}),a.on(&quot;executor-err&quot;,function(t,e,n){t[1](n)}),c.inPlace(f.prototype,[&quot;then&quot;],&quot;then-&quot;,o),a.on(&quot;then-start&quot;,function(t,e){this.promise=e,t[0]=c(t[0],&quot;cb-&quot;,this),t[1]=c(t[1],&quot;cb-&quot;,this)}),a.on(&quot;then-end&quot;,function(t,e,n){this.nextPromise=n;var r=this.promise;a.emit(&quot;propagate&quot;,[r,!0],n)}),a.on(&quot;cb-end&quot;,function(t,e,n){a.emit(&quot;propagate&quot;,[n,!0],this.nextPromise)}),a.on(&quot;propagate&quot;,function(t,e,n){this.getCtx&amp;&amp;!e||(this.getCtx=function(){if(t instanceof Promise)var e=a.context(t);return e&amp;&amp;e.getCtx?e.getCtx():this})}),r.toString=function(){return&quot;&quot;+f})},{}],12:[function(t,e,n){var r=t(&quot;ee&quot;).get(&quot;raf&quot;),o=t(23)(r),i=&quot;equestAnimationFrame&quot;;e.exports=r,o.inPlace(window,[&quot;r&quot;+i,&quot;mozR&quot;+i,&quot;webkitR&quot;+i,&quot;msR&quot;+i],&quot;raf-&quot;),r.on(&quot;raf-start&quot;,function(t){t[0]=o(t[0],&quot;fn-&quot;)})},{}],13:[function(t,e,n){function r(t,e,n){t[0]=a(t[0],&quot;fn-&quot;,null,n)}function o(t,e,n){this.method=n,this.timerDuration=isNaN(t[1])?0:+t[1],t[0]=a(t[0],&quot;fn-&quot;,this,n)}var i=t(&quot;ee&quot;).get(&quot;timer&quot;),a=t(23)(i),c=&quot;setTimeout&quot;,s=&quot;setInterval&quot;,f=&quot;clearTimeout&quot;,u=&quot;-start&quot;,d=&quot;-&quot;;e.exports=i,a.inPlace(window,[c,&quot;setImmediate&quot;],c+d),a.inPlace(window,[s],s+d),a.inPlace(window,[f,&quot;clearImmediate&quot;],f+d),i.on(s+u,r),i.on(c+u,o)},{}],14:[function(t,e,n){function r(t,e){d.inPlace(e,[&quot;onreadystatechange&quot;],&quot;fn-&quot;,c)}function o(){var t=this,e=u.context(t);t.readyState>3&amp;&amp;!e.resolved&amp;&amp;(e.resolved=!0,u.emit(&quot;xhr-resolved&quot;,[],t)),d.inPlace(t,y,&quot;fn-&quot;,c)}function i(t){b.push(t),l&amp;&amp;(x?x.then(a):v?v(a):(E=-E,P.data=E))}function a(){for(var t=0;t&lt;b.length;t++)r([],b[t]);b.length&amp;&amp;(b=[])}function c(t,e){return e}function s(t,e){for(var n in t)e[n]=t[n];return e}t(6);var f=t(&quot;ee&quot;),u=f.get(&quot;xhr&quot;),d=t(23)(u),p=NREUM.o,h=p.XHR,l=p.MO,m=p.PR,v=p.SI,w=&quot;readystatechange&quot;,y=[&quot;onload&quot;,&quot;onerror&quot;,&quot;onabort&quot;,&quot;onloadstart&quot;,&quot;onloadend&quot;,&quot;onprogress&quot;,&quot;ontimeout&quot;],b=[];e.exports=u;var g=window.XMLHttpRequest=function(t){var e=new h(t);try{u.emit(&quot;new-xhr&quot;,[e],e),e.addEventListener(w,o,!1)}catch(n){try{u.emit(&quot;internal-error&quot;,[n])}catch(r){}}return e};if(s(h,g),g.prototype=h.prototype,d.inPlace(g.prototype,[&quot;open&quot;,&quot;send&quot;],&quot;-xhr-&quot;,c),u.on(&quot;send-xhr-start&quot;,function(t,e){r(t,e),i(e)}),u.on(&quot;open-xhr-start&quot;,r),l){var x=m&amp;&amp;m.resolve();if(!v&amp;&amp;!m){var E=1,P=document.createTextNode(E);new l(a).observe(P,{characterData:!0})}}else f.on(&quot;fn-end&quot;,function(t){t[0]&amp;&amp;t[0].type===w||a()})},{}],15:[function(t,e,n){function r(t){var e=this.params,n=this.metrics;if(!this.ended){this.ended=!0;for(var r=0;r&lt;d;r++)t.removeEventListener(u[r],this.listener,!1);if(!e.aborted){if(n.duration=a.now()-this.startTime,4===t.readyState){e.status=t.status;var i=o(t,this.lastSize);if(i&amp;&amp;(n.rxSize=i),this.sameOrigin){var s=t.getResponseHeader(&quot;X-NewRelic-App-Data&quot;);s&amp;&amp;(e.cat=s.split(&quot;, &quot;).pop())}}else e.status=0;n.cbTime=this.cbTime,f.emit(&quot;xhr-done&quot;,[t],t),c(&quot;xhr&quot;,[e,n,this.startTime])}}}function o(t,e){var n=t.responseType;if(&quot;json&quot;===n&amp;&amp;null!==e)return e;var r=&quot;arraybuffer&quot;===n||&quot;blob&quot;===n||&quot;json&quot;===n?t.response:t.responseText;return l(r)}function i(t,e){var n=s(e),r=t.params;r.host=n.hostname+&quot;:&quot;+n.port,r.pathname=n.pathname,t.sameOrigin=n.sameOrigin}var a=t(&quot;loader&quot;);if(a.xhrWrappable){var c=t(&quot;handle&quot;),s=t(16),f=t(&quot;ee&quot;),u=[&quot;load&quot;,&quot;error&quot;,&quot;abort&quot;,&quot;timeout&quot;],d=u.length,p=t(&quot;id&quot;),h=t(19),l=t(18),m=window.XMLHttpRequest;a.features.xhr=!0,t(14),f.on(&quot;new-xhr&quot;,function(t){var e=this;e.totalCbs=0,e.called=0,e.cbTime=0,e.end=r,e.ended=!1,e.xhrGuids={},e.lastSize=null,h&amp;&amp;(h>34||h&lt;10)||window.opera||t.addEventListener(&quot;progress&quot;,function(t){e.lastSize=t.loaded},!1)}),f.on(&quot;open-xhr-start&quot;,function(t){this.params={method:t[0]},i(this,t[1]),this.metrics={}}),f.on(&quot;open-xhr-end&quot;,function(t,e){&quot;loader_config&quot;in NREUM&amp;&amp;&quot;xpid&quot;in NREUM.loader_config&amp;&amp;this.sameOrigin&amp;&amp;e.setRequestHeader(&quot;X-NewRelic-ID&quot;,NREUM.loader_config.xpid)}),f.on(&quot;send-xhr-start&quot;,function(t,e){var n=this.metrics,r=t[0],o=this;if(n&amp;&amp;r){var i=l(r);i&amp;&amp;(n.txSize=i)}this.startTime=a.now(),this.listener=function(t){try{&quot;abort&quot;===t.type&amp;&amp;(o.params.aborted=!0),(&quot;load&quot;!==t.type||o.called===o.totalCbs&amp;&amp;(o.onloadCalled||&quot;function&quot;!=typeof e.onload))&amp;&amp;o.end(e)}catch(n){try{f.emit(&quot;internal-error&quot;,[n])}catch(r){}}};for(var c=0;c&lt;d;c++)e.addEventListener(u[c],this.listener,!1)}),f.on(&quot;xhr-cb-time&quot;,function(t,e,n){this.cbTime+=t,e?this.onloadCalled=!0:this.called+=1,this.called!==this.totalCbs||!this.onloadCalled&amp;&amp;&quot;function&quot;==typeof n.onload||this.end(n)}),f.on(&quot;xhr-load-added&quot;,function(t,e){var n=&quot;&quot;+p(t)+!!e;this.xhrGuids&amp;&amp;!this.xhrGuids[n]&amp;&amp;(this.xhrGuids[n]=!0,this.totalCbs+=1)}),f.on(&quot;xhr-load-removed&quot;,function(t,e){var n=&quot;&quot;+p(t)+!!e;this.xhrGuids&amp;&amp;this.xhrGuids[n]&amp;&amp;(delete this.xhrGuids[n],this.totalCbs-=1)}),f.on(&quot;addEventListener-end&quot;,function(t,e){e instanceof m&amp;&amp;&quot;load&quot;===t[0]&amp;&amp;f.emit(&quot;xhr-load-added&quot;,[t[1],t[2]],e)}),f.on(&quot;removeEventListener-end&quot;,function(t,e){e instanceof m&amp;&amp;&quot;load&quot;===t[0]&amp;&amp;f.emit(&quot;xhr-load-removed&quot;,[t[1],t[2]],e)}),f.on(&quot;fn-start&quot;,function(t,e,n){e instanceof m&amp;&amp;(&quot;onload&quot;===n&amp;&amp;(this.onload=!0),(&quot;load&quot;===(t[0]&amp;&amp;t[0].type)||this.onload)&amp;&amp;(this.xhrCbStart=a.now()))}),f.on(&quot;fn-end&quot;,function(t,e){this.xhrCbStart&amp;&amp;f.emit(&quot;xhr-cb-time&quot;,[a.now()-this.xhrCbStart,this.onload,e],e)})}},{}],16:[function(t,e,n){e.exports=function(t){var e=document.createElement(&quot;a&quot;),n=window.location,r={};e.href=t,r.port=e.port;var o=e.href.split(&quot;://&quot;);!r.port&amp;&amp;o[1]&amp;&amp;(r.port=o[1].split(&quot;/&quot;)[0].split(&quot;@&quot;).pop().split(&quot;:&quot;)[1]),r.port&amp;&amp;&quot;0&quot;!==r.port||(r.port=&quot;https&quot;===o[0]?&quot;443&quot;:&quot;80&quot;),r.hostname=e.hostname||n.hostname,r.pathname=e.pathname,r.protocol=o[0],&quot;/&quot;!==r.pathname.charAt(0)&amp;&amp;(r.pathname=&quot;/&quot;+r.pathname);var i=!e.protocol||&quot;:&quot;===e.protocol||e.protocol===n.protocol,a=e.hostname===document.domain&amp;&amp;e.port===n.port;return r.sameOrigin=i&amp;&amp;(!e.hostname||a),r}},{}],17:[function(t,e,n){function r(){}function o(t,e,n){return function(){return i(t,[f.now()].concat(c(arguments)),e?null:this,n),e?void 0:this}}var i=t(&quot;handle&quot;),a=t(20),c=t(21),s=t(&quot;ee&quot;).get(&quot;tracer&quot;),f=t(&quot;loader&quot;),u=NREUM;&quot;undefined&quot;==typeof window.newrelic&amp;&amp;(newrelic=u);var d=[&quot;setPageViewName&quot;,&quot;setCustomAttribute&quot;,&quot;setErrorHandler&quot;,&quot;finished&quot;,&quot;addToTrace&quot;,&quot;inlineHit&quot;,&quot;addRelease&quot;],p=&quot;api-&quot;,h=p+&quot;ixn-&quot;;a(d,function(t,e){u[e]=o(p+e,!0,&quot;api&quot;)}),u.addPageAction=o(p+&quot;addPageAction&quot;,!0),u.setCurrentRouteName=o(p+&quot;routeName&quot;,!0),e.exports=newrelic,u.interaction=function(){return(new r).get()};var l=r.prototype={createTracer:function(t,e){var n={},r=this,o=&quot;function&quot;==typeof e;return i(h+&quot;tracer&quot;,[f.now(),t,n],r),function(){if(s.emit((o?&quot;&quot;:&quot;no-&quot;)+&quot;fn-start&quot;,[f.now(),r,o],n),o)try{return e.apply(this,arguments)}catch(t){throw s.emit(&quot;fn-err&quot;,[arguments,this,t],n),t}finally{s.emit(&quot;fn-end&quot;,[f.now()],n)}}}};a(&quot;setName,setAttribute,save,ignore,onEnd,getContext,end,get&quot;.split(&quot;,&quot;),function(t,e){l[e]=o(h+e)}),newrelic.noticeError=function(t){&quot;string&quot;==typeof t&amp;&amp;(t=new Error(t)),i(&quot;err&quot;,[t,f.now()])}},{}],18:[function(t,e,n){e.exports=function(t){if(&quot;string&quot;==typeof t&amp;&amp;t.length)return t.length;if(&quot;object&quot;==typeof t){if(&quot;undefined&quot;!=typeof ArrayBuffer&amp;&amp;t instanceof ArrayBuffer&amp;&amp;t.byteLength)return t.byteLength;if(&quot;undefined&quot;!=typeof Blob&amp;&amp;t instanceof Blob&amp;&amp;t.size)return t.size;if(!(&quot;undefined&quot;!=typeof FormData&amp;&amp;t instanceof FormData))try{return JSON.stringify(t).length}catch(e){return}}}},{}],19:[function(t,e,n){var r=0,o=navigator.userAgent.match(/Firefox[\/\s](\d+\.\d+)/);o&amp;&amp;(r=+o[1]),e.exports=r},{}],20:[function(t,e,n){function r(t,e){var n=[],r=&quot;&quot;,i=0;for(r in t)o.call(t,r)&amp;&amp;(n[i]=e(r,t[r]),i+=1);return n}var o=Object.prototype.hasOwnProperty;e.exports=r},{}],21:[function(t,e,n){function r(t,e,n){e||(e=0),&quot;undefined&quot;==typeof n&amp;&amp;(n=t?t.length:0);for(var r=-1,o=n-e||0,i=Array(o&lt;0?0:o);++r&lt;o;)i[r]=t[e+r];return i}e.exports=r},{}],22:[function(t,e,n){e.exports={exists:&quot;undefined&quot;!=typeof window.performance&amp;&amp;window.performance.timing&amp;&amp;&quot;undefined&quot;!=typeof window.performance.timing.navigationStart}},{}],23:[function(t,e,n){function r(t){return!(t&amp;&amp;t instanceof Function&amp;&amp;t.apply&amp;&amp;!t[a])}var o=t(&quot;ee&quot;),i=t(21),a=&quot;nr@original&quot;,c=Object.prototype.hasOwnProperty,s=!1;e.exports=function(t,e){function n(t,e,n,o){function nrWrapper(){var r,a,c,s;try{a=this,r=i(arguments),c=&quot;function&quot;==typeof n?n(r,a):n||{}}catch(f){p([f,&quot;&quot;,[r,a,o],c])}u(e+&quot;start&quot;,[r,a,o],c);try{return s=t.apply(a,r)}catch(d){throw u(e+&quot;err&quot;,[r,a,d],c),d}finally{u(e+&quot;end&quot;,[r,a,s],c)}}return r(t)?t:(e||(e=&quot;&quot;),nrWrapper[a]=t,d(t,nrWrapper),nrWrapper)}function f(t,e,o,i){o||(o=&quot;&quot;);var a,c,s,f=&quot;-&quot;===o.charAt(0);for(s=0;s&lt;e.length;s++)c=e[s],a=t[c],r(a)||(t[c]=n(a,f?c+o:o,i,c))}function u(n,r,o){if(!s||e){var i=s;s=!0;try{t.emit(n,r,o,e)}catch(a){p([a,n,r,o])}s=i}}function d(t,e){if(Object.defineProperty&amp;&amp;Object.keys)try{var n=Object.keys(t);return n.forEach(function(n){Object.defineProperty(e,n,{get:function(){return t[n]},set:function(e){return t[n]=e,e}})}),e}catch(r){p([r])}for(var o in t)c.call(t,o)&amp;&amp;(e[o]=t[o]);return e}function p(e){try{t.emit(&quot;internal-error&quot;,e)}catch(n){}}return t||(t=o),n.inPlace=f,n.flag=a,n}},{}],ee:[function(t,e,n){function r(){}function o(t){function e(t){return t&amp;&amp;t instanceof r?t:t?s(t,c,i):i()}function n(n,r,o,i){if(!p.aborted||i){t&amp;&amp;t(n,r,o);for(var a=e(o),c=l(n),s=c.length,f=0;f&lt;s;f++)c[f].apply(a,r);var d=u[y[n]];return d&amp;&amp;d.push([b,n,r,a]),a}}function h(t,e){w[t]=l(t).concat(e)}function l(t){return w[t]||[]}function m(t){return d[t]=d[t]||o(n)}function v(t,e){f(t,function(t,n){e=e||&quot;feature&quot;,y[n]=e,e in u||(u[e]=[])})}var w={},y={},b={on:h,emit:n,get:m,listeners:l,context:e,buffer:v,abort:a,aborted:!1};return b}function i(){return new r}function a(){(u.api||u.feature)&amp;&amp;(p.aborted=!0,u=p.backlog={})}var c=&quot;nr@context&quot;,s=t(&quot;gos&quot;),f=t(20),u={},d={},p=e.exports=o();p.backlog=u},{}],gos:[function(t,e,n){function r(t,e,n){if(o.call(t,e))return t[e];var r=n();if(Object.defineProperty&amp;&amp;Object.keys)try{return Object.defineProperty(t,e,{value:r,writable:!0,enumerable:!1}),r}catch(i){}return t[e]=r,r}var o=Object.prototype.hasOwnProperty;e.exports=r},{}],handle:[function(t,e,n){function r(t,e,n,r){o.buffer([t],r),o.emit(t,e,n)}var o=t(&quot;ee&quot;).get(&quot;handle&quot;);e.exports=r,r.ee=o},{}],id:[function(t,e,n){function r(t){var e=typeof t;return!t||&quot;object&quot;!==e&amp;&amp;&quot;function&quot;!==e?-1:t===window?0:a(t,i,function(){return o++})}var o=1,i=&quot;nr@id&quot;,a=t(&quot;gos&quot;);e.exports=r},{}],loader:[function(t,e,n){function r(){if(!x++){var t=g.info=NREUM.info,e=p.getElementsByTagName(&quot;script&quot;)[0];if(setTimeout(u.abort,3e4),!(t&amp;&amp;t.licenseKey&amp;&amp;t.applicationID&amp;&amp;e))return u.abort();f(y,function(e,n){t[e]||(t[e]=n)}),s(&quot;mark&quot;,[&quot;onload&quot;,a()+g.offset],null,&quot;api&quot;);var n=p.createElement(&quot;script&quot;);n.src=&quot;https://&quot;+t.agent,e.parentNode.insertBefore(n,e)}}function o(){&quot;complete&quot;===p.readyState&amp;&amp;i()}function i(){s(&quot;mark&quot;,[&quot;domContent&quot;,a()+g.offset],null,&quot;api&quot;)}function a(){return E.exists&amp;&amp;performance.now?Math.round(performance.now()):(c=Math.max((new Date).getTime(),c))-g.offset}var c=(new Date).getTime(),s=t(&quot;handle&quot;),f=t(20),u=t(&quot;ee&quot;),d=window,p=d.document,h=&quot;addEventListener&quot;,l=&quot;attachEvent&quot;,m=d.XMLHttpRequest,v=m&amp;&amp;m.prototype;NREUM.o={ST:setTimeout,SI:d.setImmediate,CT:clearTimeout,XHR:m,REQ:d.Request,EV:d.Event,PR:d.Promise,MO:d.MutationObserver};var w=&quot;&quot;+location,y={beacon:&quot;bam.nr-data.net&quot;,errorBeacon:&quot;bam.nr-data.net&quot;,agent:&quot;js-agent.newrelic.com/nr-spa-1071.min.js&quot;},b=m&amp;&amp;v&amp;&amp;v[h]&amp;&amp;!/CriOS/.test(navigator.userAgent),g=e.exports={offset:c,now:a,origin:w,features:{},xhrWrappable:b};t(17),p[h]?(p[h](&quot;DOMContentLoaded&quot;,i,!1),d[h](&quot;load&quot;,r,!1)):(p[l](&quot;onreadystatechange&quot;,o),d[l](&quot;onload&quot;,r)),s(&quot;mark&quot;,[&quot;firstbyte&quot;,c],null,&quot;api&quot;);var x=0,E=t(22)},{}]},{},[&quot;loader&quot;,2,15,5,3,4]);



	

	

	

	

	

	

	

	

	

	

	

	

	

	





#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-elementInfoDiv {color: lightblue; padding: 5px}

	
		















    


    var contextPath = '/DCM_UI';
    var servletPath = '/create-location-group';
    var userId = 'p165114';
    var userRole = 'ADMIN';
    var userName = 'Peters,Carrol';
    var sessionTimeout = '1800';

    function formSubmit() {
        document.getElementById(&quot;logoutForm&quot;).submit();
    }


       Warning! Due to inactivity, your session will expire in 00:00:00. To extend your session another 30 minute(s), please press the Extend button.
    Extend


   Warning! Due to inactivity, your session has expired. Please save any data you may have entered before refreshing the page.


    
        
            
        
        Cost and Deals
    
    
        
            
                
                    
                    
                    
                
                Logged in as
                    
                    
                    
                    Peters,Carrol
                    
                    
                
                Sign Out
            
        
        
            
                Toggle navigation
                
                
                
            
        
        
            
                
                    Home
                    
                        
                            Switch To...
                            Announcements
                        
                    
                    
                
                
                    Offer
                    
                        
                            Create
                        
                        Deal Builder
                        Review
                        
                            Accept/Reject
                        
                        Search
                        
                            Maintain
                            Mass Upload
                        
                    
                
                Deals
                    
                        Search
                        Maintain
                        SODL
                            
                                Search &amp; Maintenance
                                Details
                                Transactions
                            
                        
                        Browse Deals and Costs
                    
                
                Cost
                    
                        Maintain
                        Browse Deals and Costs
                    
                
                Location Group
                    
                        
                            Create
                        
                        Search
                        
                            Maintain
                        
                    
                
                Items
                    
                        Cost Link Maintenance
                    
                
                
                    People
                        
                            User Preferences
                            Forward Alert and Share Authority
                        
                    
                
                
                
                Help
                
            
        
    
    
        
            
            
            
            
        
        
            
                
                    Help
                    
                
            

            
                
                    
                
                
                    
                
            
        


        
        
            
            	
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                    
                    HEB Enterprise Portal
                    Cost and Deals
                    Location Group
                    Create
                
                
                
                
                
                
                
                
                
            
        
    

            
            	
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                    
                    HEB Enterprise Portal
                    Cost and Deals
                    Location Group
                    Create
                
                
                
                
                
                
                
                
                
            
        

    
        
            
                User Setting
                ×
            
            
                
                    General
                    Custom Settings
                
                
                    
                        
                        
                            Dashboard
                            
                                 Show Information Alerts
                            
                            
                                 Activate &quot;Switch to&quot; User
                            
                            
                                Switch to User Id 
                                
                                    
                                
                                
                            
                        
                        
                            Available Partners
                            
                        
                        
                    
                    
                        
                        
                            Screen
                            
                                 Application 
                            
                            
                                 Screen Name 
                            
                            
                                 Tab Name 
                            
                        
                        
                            Misc.
                            
                                 Default Start Date to 
                                
                                 Days Prior to Today 
                            
                        
                        
                            Available Columns
                            
                        
                        
                    
                
                
                    OK
                    Cancel
                    Reset to Defaults
                
            
        
    


    var highlightMenuItem = 'basket-create';
    document.addEventListener(&quot;DOMContentLoaded&quot;, function(event) {
        if (highlightMenuItem !== '') {
            $('*[data-highlight=&quot;' + highlightMenuItem + '&quot;]').addClass('active');
            $('*[data-highlight=&quot;' + highlightMenuItem + '&quot;]').closest('li.dropdown').addClass('active');
        }
    });


		
			
	
		CREATE LOCATION GROUP
		
			
				
			
			
				
			
			
				
			
			
				
			
			
				
			
		
	
	
		
		
		
			
				
					
						The following errors were encountered:
						
					
					
						
					
				
				
			
		
		
		
			
				Location Group # or Name
			
			
				
				
					
						Name:
						add new
					
					
						Owner:
						p165114 Peters,Carrol
					
					
						Domain:
						HEB - HEB ONLY PARTNERS
					
					
						Type:
						LOCATION
					
				

			
			
				
					
						Details for Location Group #
						
					
				
				

					
						
							
								Note: Required fields are marked with an asterisk (*)
							
							
								
								
									
										Group Name
									
									
										
										
											*
										
									
								
								
								
								
									
										Owner
									
									
										
										
											*
										
									
									
										Peters,Carrol
									
								
								
								
								
									
										Domain
									
									
										
											HEB - HEB ONLY PARTNERS - Select -HEB - HEB ONLY PARTNERSVEND - BOTH HEB AND VENDORS- Select -HEB - HEB ONLY PARTNERSVEND - BOTH HEB AND VENDORS
											
										
										
											*
										
									
								
								
								
								
									
										Type
									
									
										STORE - LOCATION
										
										
									
								

								
									
										Public
									
									
										
									
								

								
									
										Make Location Group usable by
									
									
										
									
									
										
									
								

							
						
						
							
								
									 
								
								
									
										Abbreviation
									
									
										
										
											*
										
									
								
								
								
									
										Description
									
									
										
											
										
									
								
								
								
									
										Level
									
									
										
											USER - USER DEFINED CGRP - COST GROUPCORP - CORPORATE DERIVEDUSER - USER DEFINEDCGRP - COST GROUPCORP - CORPORATE DERIVEDUSER - USER DEFINED
											
										
									
								
								
								
									
										No of Elements
									
									
										0
									
								
								
									
										Active
									
									
										
									
								
							
						

						
							
								
									 
								
								
									
										Public Comments
									
									
										
									
								
								
									
										Private Comments
									
									
										
									
								
							
							
								
									
										Created By
									
									
										
									
								
							
							
								
									
										Updated By
									
									
										
									
								
							
						
					
				
			
		
		
			
				
					
				
			
			
				
					
						
							Select
						
						
						
							
								
								
								
							
							
							
								
							
						
						
							All Stores
								
									
								00000 - 00001 - BR01 ELIZABETH00006 - STEPHENVILLE00009 - DONNA00013 - RIO GRANDE CITY-HWY83/S BRIDGE00014 - KYLE00016 - BURLESON - JOHN JONES/WILSHIRE00017 - ANGLETON00019 - LYTLE    FM 2790/IH3500020 - H53 FM 290/BARKER CYPRESS
							
						
						
							Page 1 of 39
							Total items 386
						
						
							12345Next >Last >>
						
					
				
				
					
						
					
					
						
					
					
						
					
				
				
					
						
							Add to Location Group
						
						
						
							
								
								
							
							
							
								
							
						
						
							
								
									
								
							
						
					
				
			
		
		

		
			
				
					
						
							Help
							
						
					

				
			
		

	
		var basketIdFromBasketSearch = &quot;0&quot;; //int
		var isCopyFromBasketSearch = &quot;&quot;;
	

		
	
	
	

window.NREUM||(NREUM={});NREUM.info={&quot;errorBeacon&quot;:&quot;bam.nr-data.net&quot;,&quot;licenseKey&quot;:&quot;107f4f8ae4&quot;,&quot;agent&quot;:&quot;&quot;,&quot;beacon&quot;:&quot;bam.nr-data.net&quot;,&quot;applicationTime&quot;:6,&quot;applicationID&quot;:&quot;108906967&quot;,&quot;transactionName&quot;:&quot;NVNSN0dTXkRTBkFZDQwZYxNHW15QcQpbRBANWlwGRx1TRVcEQVVPDllTAkFbX1kfAkdfFxIWGCRwZhk=&quot;,&quot;queueTime&quot;:0}


	

	

	

	

	

	

	

	

	

	

	

	

	

	

	

	

	

	

	

	

	

	



	

	

	


/html[1]</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]</value>
   </webElementProperties>
</WebElementEntity>
