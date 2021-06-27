import{X as t,Y as e,Z as r,$ as o,m as n,_ as l,q as i,e as a,B as c,h as s,j as d,A as h,b as u,d as p,o as f,p as _}from"./core.js";import{Tab as g}from"./mwc-tab.js";var m={ANIMATING:"mdc-tab-scroller--animating",SCROLL_AREA_SCROLL:"mdc-tab-scroller__scroll-area--scroll",SCROLL_TEST:"mdc-tab-scroller__test"},S={AREA_SELECTOR:".mdc-tab-scroller__scroll-area",CONTENT_SELECTOR:".mdc-tab-scroller__scroll-content"},T=function(e){function r(){return null!==e&&e.apply(this,arguments)||this}return t(r,e),r.prototype.getScrollPositionRTL=function(){var t=this.adapter.getScrollAreaScrollLeft(),e=this.calculateScrollEdges_().right;return Math.round(e-t)},r.prototype.scrollToRTL=function(t){var e=this.calculateScrollEdges_(),r=this.adapter.getScrollAreaScrollLeft(),o=this.clampScrollValue_(e.right-t);return{finalScrollPosition:o,scrollDelta:o-r}},r.prototype.incrementScrollRTL=function(t){var e=this.adapter.getScrollAreaScrollLeft(),r=this.clampScrollValue_(e-t);return{finalScrollPosition:r,scrollDelta:r-e}},r.prototype.getAnimatingScrollPosition=function(t){return t},r.prototype.calculateScrollEdges_=function(){return{left:0,right:this.adapter.getScrollContentOffsetWidth()-this.adapter.getScrollAreaOffsetWidth()}},r.prototype.clampScrollValue_=function(t){var e=this.calculateScrollEdges_();return Math.min(Math.max(e.left,t),e.right)},r}(e),A=function(e){function r(){return null!==e&&e.apply(this,arguments)||this}return t(r,e),r.prototype.getScrollPositionRTL=function(t){var e=this.adapter.getScrollAreaScrollLeft();return Math.round(t-e)},r.prototype.scrollToRTL=function(t){var e=this.adapter.getScrollAreaScrollLeft(),r=this.clampScrollValue_(-t);return{finalScrollPosition:r,scrollDelta:r-e}},r.prototype.incrementScrollRTL=function(t){var e=this.adapter.getScrollAreaScrollLeft(),r=this.clampScrollValue_(e-t);return{finalScrollPosition:r,scrollDelta:r-e}},r.prototype.getAnimatingScrollPosition=function(t,e){return t-e},r.prototype.calculateScrollEdges_=function(){var t=this.adapter.getScrollContentOffsetWidth();return{left:this.adapter.getScrollAreaOffsetWidth()-t,right:0}},r.prototype.clampScrollValue_=function(t){var e=this.calculateScrollEdges_();return Math.max(Math.min(e.right,t),e.left)},r}(e),b=function(e){function r(){return null!==e&&e.apply(this,arguments)||this}return t(r,e),r.prototype.getScrollPositionRTL=function(t){var e=this.adapter.getScrollAreaScrollLeft();return Math.round(e-t)},r.prototype.scrollToRTL=function(t){var e=this.adapter.getScrollAreaScrollLeft(),r=this.clampScrollValue_(t);return{finalScrollPosition:r,scrollDelta:e-r}},r.prototype.incrementScrollRTL=function(t){var e=this.adapter.getScrollAreaScrollLeft(),r=this.clampScrollValue_(e+t);return{finalScrollPosition:r,scrollDelta:e-r}},r.prototype.getAnimatingScrollPosition=function(t,e){return t+e},r.prototype.calculateScrollEdges_=function(){return{left:this.adapter.getScrollContentOffsetWidth()-this.adapter.getScrollAreaOffsetWidth(),right:0}},r.prototype.clampScrollValue_=function(t){var e=this.calculateScrollEdges_();return Math.min(Math.max(e.right,t),e.left)},r}(e),E=function(e){function n(t){var r=e.call(this,o(o({},n.defaultAdapter),t))||this;return r.isAnimating_=!1,r}return t(n,e),Object.defineProperty(n,"cssClasses",{get:function(){return m},enumerable:!0,configurable:!0}),Object.defineProperty(n,"strings",{get:function(){return S},enumerable:!0,configurable:!0}),Object.defineProperty(n,"defaultAdapter",{get:function(){return{eventTargetMatchesSelector:function(){return!1},addClass:function(){},removeClass:function(){},addScrollAreaClass:function(){},setScrollAreaStyleProperty:function(){},setScrollContentStyleProperty:function(){},getScrollContentStyleValue:function(){return""},setScrollAreaScrollLeft:function(){},getScrollAreaScrollLeft:function(){return 0},getScrollContentOffsetWidth:function(){return 0},getScrollAreaOffsetWidth:function(){return 0},computeScrollAreaClientRect:function(){return{top:0,right:0,bottom:0,left:0,width:0,height:0}},computeScrollContentClientRect:function(){return{top:0,right:0,bottom:0,left:0,width:0,height:0}},computeHorizontalScrollbarHeight:function(){return 0}}},enumerable:!0,configurable:!0}),n.prototype.init=function(){var t=this.adapter.computeHorizontalScrollbarHeight();this.adapter.setScrollAreaStyleProperty("margin-bottom",-t+"px"),this.adapter.addScrollAreaClass(n.cssClasses.SCROLL_AREA_SCROLL)},n.prototype.getScrollPosition=function(){if(this.isRTL_())return this.computeCurrentScrollPositionRTL_();var t=this.calculateCurrentTranslateX_();return this.adapter.getScrollAreaScrollLeft()-t},n.prototype.handleInteraction=function(){this.isAnimating_&&this.stopScrollAnimation_()},n.prototype.handleTransitionEnd=function(t){var e=t.target;this.isAnimating_&&this.adapter.eventTargetMatchesSelector(e,n.strings.CONTENT_SELECTOR)&&(this.isAnimating_=!1,this.adapter.removeClass(n.cssClasses.ANIMATING))},n.prototype.incrementScroll=function(t){0!==t&&this.animate_(this.getIncrementScrollOperation_(t))},n.prototype.incrementScrollImmediate=function(t){if(0!==t){var e=this.getIncrementScrollOperation_(t);0!==e.scrollDelta&&(this.stopScrollAnimation_(),this.adapter.setScrollAreaScrollLeft(e.finalScrollPosition))}},n.prototype.scrollTo=function(t){if(this.isRTL_())return this.scrollToRTL_(t);this.scrollTo_(t)},n.prototype.getRTLScroller=function(){return this.rtlScrollerInstance_||(this.rtlScrollerInstance_=this.rtlScrollerFactory_()),this.rtlScrollerInstance_},n.prototype.calculateCurrentTranslateX_=function(){var t=this.adapter.getScrollContentStyleValue("transform");if("none"===t)return 0;var e=/\((.+?)\)/.exec(t);if(!e)return 0;var o=e[1],n=r(o.split(","),6),l=(n[0],n[1],n[2],n[3],n[4]);n[5];return parseFloat(l)},n.prototype.clampScrollValue_=function(t){var e=this.calculateScrollEdges_();return Math.min(Math.max(e.left,t),e.right)},n.prototype.computeCurrentScrollPositionRTL_=function(){var t=this.calculateCurrentTranslateX_();return this.getRTLScroller().getScrollPositionRTL(t)},n.prototype.calculateScrollEdges_=function(){return{left:0,right:this.adapter.getScrollContentOffsetWidth()-this.adapter.getScrollAreaOffsetWidth()}},n.prototype.scrollTo_=function(t){var e=this.getScrollPosition(),r=this.clampScrollValue_(t),o=r-e;this.animate_({finalScrollPosition:r,scrollDelta:o})},n.prototype.scrollToRTL_=function(t){var e=this.getRTLScroller().scrollToRTL(t);this.animate_(e)},n.prototype.getIncrementScrollOperation_=function(t){if(this.isRTL_())return this.getRTLScroller().incrementScrollRTL(t);var e=this.getScrollPosition(),r=t+e,o=this.clampScrollValue_(r);return{finalScrollPosition:o,scrollDelta:o-e}},n.prototype.animate_=function(t){var e=this;0!==t.scrollDelta&&(this.stopScrollAnimation_(),this.adapter.setScrollAreaScrollLeft(t.finalScrollPosition),this.adapter.setScrollContentStyleProperty("transform","translateX("+t.scrollDelta+"px)"),this.adapter.computeScrollAreaClientRect(),requestAnimationFrame((function(){e.adapter.addClass(n.cssClasses.ANIMATING),e.adapter.setScrollContentStyleProperty("transform","none")})),this.isAnimating_=!0)},n.prototype.stopScrollAnimation_=function(){this.isAnimating_=!1;var t=this.getAnimatingScrollPosition_();this.adapter.removeClass(n.cssClasses.ANIMATING),this.adapter.setScrollContentStyleProperty("transform","translateX(0px)"),this.adapter.setScrollAreaScrollLeft(t)},n.prototype.getAnimatingScrollPosition_=function(){var t=this.calculateCurrentTranslateX_(),e=this.adapter.getScrollAreaScrollLeft();return this.isRTL_()?this.getRTLScroller().getAnimatingScrollPosition(e,t):e-t},n.prototype.rtlScrollerFactory_=function(){var t=this.adapter.getScrollAreaScrollLeft();this.adapter.setScrollAreaScrollLeft(t-1);var e=this.adapter.getScrollAreaScrollLeft();if(e<0)return this.adapter.setScrollAreaScrollLeft(t),new A(this.adapter);var r=this.adapter.computeScrollAreaClientRect(),o=this.adapter.computeScrollContentClientRect(),n=Math.round(o.right-r.right);return this.adapter.setScrollAreaScrollLeft(t),n===e?new b(this.adapter):new T(this.adapter)},n.prototype.isRTL_=function(){return"rtl"===this.adapter.getScrollContentStyleValue("direction")},n}(n);class y extends c{constructor(){super(...arguments),this.mdcFoundationClass=E,this._scrollbarHeight=-1}_handleInteraction(){this.mdcFoundation.handleInteraction()}_handleTransitionEnd(t){this.mdcFoundation.handleTransitionEnd(t)}render(){return s`
      <div class="mdc-tab-scroller">
        <div class="mdc-tab-scroller__scroll-area"
            @wheel="${this._handleInteraction}"
            @touchstart="${this._handleInteraction}"
            @pointerdown="${this._handleInteraction}"
            @mousedown="${this._handleInteraction}"
            @keydown="${this._handleInteraction}"
            @transitionend="${this._handleTransitionEnd}">
          <div class="mdc-tab-scroller__scroll-content"><slot></slot></div>
        </div>
      </div>
      `}createAdapter(){return Object.assign(Object.assign({},d(this.mdcRoot)),{eventTargetMatchesSelector:(t,e)=>h(t,e),addScrollAreaClass:t=>this.scrollAreaElement.classList.add(t),setScrollAreaStyleProperty:(t,e)=>this.scrollAreaElement.style.setProperty(t,e),setScrollContentStyleProperty:(t,e)=>this.scrollContentElement.style.setProperty(t,e),getScrollContentStyleValue:t=>window.getComputedStyle(this.scrollContentElement).getPropertyValue(t),setScrollAreaScrollLeft:t=>this.scrollAreaElement.scrollLeft=t,getScrollAreaScrollLeft:()=>this.scrollAreaElement.scrollLeft,getScrollContentOffsetWidth:()=>this.scrollContentElement.offsetWidth,getScrollAreaOffsetWidth:()=>this.scrollAreaElement.offsetWidth,computeScrollAreaClientRect:()=>this.scrollAreaElement.getBoundingClientRect(),computeScrollContentClientRect:()=>this.scrollContentElement.getBoundingClientRect(),computeHorizontalScrollbarHeight:()=>(-1===this._scrollbarHeight&&(this.scrollAreaElement.style.overflowX="scroll",this._scrollbarHeight=this.scrollAreaElement.offsetHeight-this.scrollAreaElement.clientHeight,this.scrollAreaElement.style.overflowX=""),this._scrollbarHeight)})}getScrollPosition(){return this.mdcFoundation.getScrollPosition()}getScrollContentWidth(){return this.scrollContentElement.offsetWidth}incrementScrollPosition(t){this.mdcFoundation.incrementScroll(t)}scrollToPosition(t){this.mdcFoundation.scrollTo(t)}}l([i(".mdc-tab-scroller")],y.prototype,"mdcRoot",void 0),l([i(".mdc-tab-scroller__scroll-area")],y.prototype,"scrollAreaElement",void 0),l([i(".mdc-tab-scroller__scroll-content")],y.prototype,"scrollContentElement",void 0),l([a({passive:!0})],y.prototype,"_handleInteraction",null);const v=u`.mdc-tab-scroller{overflow-y:hidden}.mdc-tab-scroller.mdc-tab-scroller--animating .mdc-tab-scroller__scroll-content{transition:250ms transform cubic-bezier(0.4, 0, 0.2, 1)}.mdc-tab-scroller__test{position:absolute;top:-9999px;width:100px;height:100px;overflow-x:scroll}.mdc-tab-scroller__scroll-area{-webkit-overflow-scrolling:touch;display:flex;overflow-x:hidden}.mdc-tab-scroller__scroll-area::-webkit-scrollbar,.mdc-tab-scroller__test::-webkit-scrollbar{display:none}.mdc-tab-scroller__scroll-area--scroll{overflow-x:scroll}.mdc-tab-scroller__scroll-content{position:relative;display:flex;flex:1 0 auto;transform:none;will-change:transform}.mdc-tab-scroller--align-start .mdc-tab-scroller__scroll-content{justify-content:flex-start}.mdc-tab-scroller--align-end .mdc-tab-scroller__scroll-content{justify-content:flex-end}.mdc-tab-scroller--align-center .mdc-tab-scroller__scroll-content{justify-content:center}.mdc-tab-scroller--animating .mdc-tab-scroller__scroll-area{-webkit-overflow-scrolling:auto}:host{display:flex}.mdc-tab-scroller{flex:1}`;let R=class extends y{};R.styles=v,R=l([p("mwc-tab-scroller")],R);var L=function(t,e){return(L=Object.setPrototypeOf||{__proto__:[]}instanceof Array&&function(t,e){t.__proto__=e}||function(t,e){for(var r in e)e.hasOwnProperty(r)&&(t[r]=e[r])})(t,e)};var C=function(){return(C=Object.assign||function(t){for(var e,r=1,o=arguments.length;r<o;r++)for(var n in e=arguments[r])Object.prototype.hasOwnProperty.call(e,n)&&(t[n]=e[n]);return t}).apply(this,arguments)},I={ARROW_LEFT_KEY:"ArrowLeft",ARROW_RIGHT_KEY:"ArrowRight",END_KEY:"End",ENTER_KEY:"Enter",HOME_KEY:"Home",SPACE_KEY:"Space",TAB_ACTIVATED_EVENT:"MDCTabBar:activated",TAB_SCROLLER_SELECTOR:".mdc-tab-scroller",TAB_SELECTOR:".mdc-tab"},O={ARROW_LEFT_KEYCODE:37,ARROW_RIGHT_KEYCODE:39,END_KEYCODE:35,ENTER_KEYCODE:13,EXTRA_SCROLL_AMOUNT:20,HOME_KEYCODE:36,SPACE_KEYCODE:32},x=new Set;x.add(I.ARROW_LEFT_KEY),x.add(I.ARROW_RIGHT_KEY),x.add(I.END_KEY),x.add(I.HOME_KEY),x.add(I.ENTER_KEY),x.add(I.SPACE_KEY);var P=new Map;P.set(O.ARROW_LEFT_KEYCODE,I.ARROW_LEFT_KEY),P.set(O.ARROW_RIGHT_KEYCODE,I.ARROW_RIGHT_KEY),P.set(O.END_KEYCODE,I.END_KEY),P.set(O.HOME_KEYCODE,I.HOME_KEY),P.set(O.ENTER_KEYCODE,I.ENTER_KEY),P.set(O.SPACE_KEYCODE,I.SPACE_KEY);var w=function(t){function e(r){var o=t.call(this,C(C({},e.defaultAdapter),r))||this;return o.useAutomaticActivation_=!1,o}return function(t,e){function r(){this.constructor=t}L(t,e),t.prototype=null===e?Object.create(e):(r.prototype=e.prototype,new r)}(e,t),Object.defineProperty(e,"strings",{get:function(){return I},enumerable:!0,configurable:!0}),Object.defineProperty(e,"numbers",{get:function(){return O},enumerable:!0,configurable:!0}),Object.defineProperty(e,"defaultAdapter",{get:function(){return{scrollTo:function(){},incrementScroll:function(){},getScrollPosition:function(){return 0},getScrollContentWidth:function(){return 0},getOffsetWidth:function(){return 0},isRTL:function(){return!1},setActiveTab:function(){},activateTabAtIndex:function(){},deactivateTabAtIndex:function(){},focusTabAtIndex:function(){},getTabIndicatorClientRectAtIndex:function(){return{top:0,right:0,bottom:0,left:0,width:0,height:0}},getTabDimensionsAtIndex:function(){return{rootLeft:0,rootRight:0,contentLeft:0,contentRight:0}},getPreviousActiveTabIndex:function(){return-1},getFocusedTabIndex:function(){return-1},getIndexOfTabById:function(){return-1},getTabListLength:function(){return 0},notifyTabActivated:function(){}}},enumerable:!0,configurable:!0}),e.prototype.setUseAutomaticActivation=function(t){this.useAutomaticActivation_=t},e.prototype.activateTab=function(t){var e,r=this.adapter.getPreviousActiveTabIndex();this.indexIsInRange_(t)&&t!==r&&(-1!==r&&(this.adapter.deactivateTabAtIndex(r),e=this.adapter.getTabIndicatorClientRectAtIndex(r)),this.adapter.activateTabAtIndex(t,e),this.scrollIntoView(t),this.adapter.notifyTabActivated(t))},e.prototype.handleKeyDown=function(t){var e=this.getKeyFromEvent_(t);if(void 0!==e)if(this.isActivationKey_(e)||t.preventDefault(),this.useAutomaticActivation_){if(this.isActivationKey_(e))return;var r=this.determineTargetFromKey_(this.adapter.getPreviousActiveTabIndex(),e);this.adapter.setActiveTab(r),this.scrollIntoView(r)}else{var o=this.adapter.getFocusedTabIndex();if(this.isActivationKey_(e))this.adapter.setActiveTab(o);else{r=this.determineTargetFromKey_(o,e);this.adapter.focusTabAtIndex(r),this.scrollIntoView(r)}}},e.prototype.handleTabInteraction=function(t){this.adapter.setActiveTab(this.adapter.getIndexOfTabById(t.detail.tabId))},e.prototype.scrollIntoView=function(t){if(this.indexIsInRange_(t))return 0===t?this.adapter.scrollTo(0):t===this.adapter.getTabListLength()-1?this.adapter.scrollTo(this.adapter.getScrollContentWidth()):this.isRTL_()?this.scrollIntoViewRTL_(t):void this.scrollIntoView_(t)},e.prototype.determineTargetFromKey_=function(t,e){var r=this.isRTL_(),o=this.adapter.getTabListLength()-1,n=t;return e===I.END_KEY?n=o:e===I.ARROW_LEFT_KEY&&!r||e===I.ARROW_RIGHT_KEY&&r?n-=1:e===I.ARROW_RIGHT_KEY&&!r||e===I.ARROW_LEFT_KEY&&r?n+=1:n=0,n<0?n=o:n>o&&(n=0),n},e.prototype.calculateScrollIncrement_=function(t,e,r,o){var n=this.adapter.getTabDimensionsAtIndex(e),l=n.contentLeft-r-o,i=n.contentRight-r-O.EXTRA_SCROLL_AMOUNT,a=l+O.EXTRA_SCROLL_AMOUNT;return e<t?Math.min(i,0):Math.max(a,0)},e.prototype.calculateScrollIncrementRTL_=function(t,e,r,o,n){var l=this.adapter.getTabDimensionsAtIndex(e),i=n-l.contentLeft-r,a=n-l.contentRight-r-o+O.EXTRA_SCROLL_AMOUNT,c=i-O.EXTRA_SCROLL_AMOUNT;return e>t?Math.max(a,0):Math.min(c,0)},e.prototype.findAdjacentTabIndexClosestToEdge_=function(t,e,r,o){var n=e.rootLeft-r,l=e.rootRight-r-o,i=n+l;return n<0||i<0?t-1:l>0||i>0?t+1:-1},e.prototype.findAdjacentTabIndexClosestToEdgeRTL_=function(t,e,r,o,n){var l=n-e.rootLeft-o-r,i=n-e.rootRight-r,a=l+i;return l>0||a>0?t+1:i<0||a<0?t-1:-1},e.prototype.getKeyFromEvent_=function(t){return x.has(t.key)?t.key:P.get(t.keyCode)},e.prototype.isActivationKey_=function(t){return t===I.SPACE_KEY||t===I.ENTER_KEY},e.prototype.indexIsInRange_=function(t){return t>=0&&t<this.adapter.getTabListLength()},e.prototype.isRTL_=function(){return this.adapter.isRTL()},e.prototype.scrollIntoView_=function(t){var e=this.adapter.getScrollPosition(),r=this.adapter.getOffsetWidth(),o=this.adapter.getTabDimensionsAtIndex(t),n=this.findAdjacentTabIndexClosestToEdge_(t,o,e,r);if(this.indexIsInRange_(n)){var l=this.calculateScrollIncrement_(t,n,e,r);this.adapter.incrementScroll(l)}},e.prototype.scrollIntoViewRTL_=function(t){var e=this.adapter.getScrollPosition(),r=this.adapter.getOffsetWidth(),o=this.adapter.getTabDimensionsAtIndex(t),n=this.adapter.getScrollContentWidth(),l=this.findAdjacentTabIndexClosestToEdgeRTL_(t,o,e,r,n);if(this.indexIsInRange_(l)){var i=this.calculateScrollIncrementRTL_(t,l,e,r,n);this.adapter.incrementScroll(i)}},e}(n);class K extends c{constructor(){super(...arguments),this.mdcFoundationClass=w,this.activeIndex=0,this._previousActiveIndex=-1}_handleTabInteraction(t){this.mdcFoundation.handleTabInteraction(t)}_handleKeydown(t){this.mdcFoundation.handleKeyDown(t)}render(){return s`
      <div class="mdc-tab-bar" role="tablist"
          @MDCTab:interacted="${this._handleTabInteraction}"
          @keydown="${this._handleKeydown}">
        <mwc-tab-scroller><slot></slot></mwc-tab-scroller>
      </div>
      `}_getTabs(){return this.tabsSlot.assignedNodes({flatten:!0}).filter(t=>t instanceof g)}_getTab(t){return this._getTabs()[t]}createAdapter(){return{scrollTo:t=>this.scrollerElement.scrollToPosition(t),incrementScroll:t=>this.scrollerElement.incrementScrollPosition(t),getScrollPosition:()=>this.scrollerElement.getScrollPosition(),getScrollContentWidth:()=>this.scrollerElement.getScrollContentWidth(),getOffsetWidth:()=>this.mdcRoot.offsetWidth,isRTL:()=>"rtl"===window.getComputedStyle(this.mdcRoot).getPropertyValue("direction"),setActiveTab:t=>this.mdcFoundation.activateTab(t),activateTabAtIndex:(t,e)=>{const r=this._getTab(t);void 0!==r&&r.activate(e),this._previousActiveIndex=t},deactivateTabAtIndex:t=>{const e=this._getTab(t);void 0!==e&&e.deactivate()},focusTabAtIndex:t=>{const e=this._getTab(t);void 0!==e&&e.focus()},getTabIndicatorClientRectAtIndex:t=>{const e=this._getTab(t);return void 0!==e?e.computeIndicatorClientRect():new DOMRect},getTabDimensionsAtIndex:t=>{const e=this._getTab(t);return void 0!==e?e.computeDimensions():{rootLeft:0,rootRight:0,contentLeft:0,contentRight:0}},getPreviousActiveTabIndex:()=>this._previousActiveIndex,getFocusedTabIndex:()=>{const t=this._getTabs(),e=this.getRootNode().activeElement;return t.indexOf(e)},getIndexOfTabById:t=>{const e=this._getTabs();for(let r=0;r<e.length;r++)if(e[r].id===t)return r;return-1},getTabListLength:()=>this._getTabs().length,notifyTabActivated:t=>{this.activeIndex=t,this.dispatchEvent(new CustomEvent(w.strings.TAB_ACTIVATED_EVENT,{detail:{index:t},bubbles:!0,cancelable:!0}))}}}firstUpdated(){}_getUpdateComplete(){return super._getUpdateComplete().then(()=>this.scrollerElement.updateComplete).then(()=>{void 0===this.mdcFoundation&&this.createFoundation()})}scrollIndexIntoView(t){this.mdcFoundation.scrollIntoView(t)}}l([i(".mdc-tab-bar")],K.prototype,"mdcRoot",void 0),l([i("mwc-tab-scroller")],K.prototype,"scrollerElement",void 0),l([i("slot")],K.prototype,"tabsSlot",void 0),l([f((async function(){await this.updateComplete,this.activeIndex!==this._previousActiveIndex&&this.mdcFoundation.activateTab(this.activeIndex)})),_({type:Number})],K.prototype,"activeIndex",void 0);const D=u`.mdc-tab-bar{width:100%}.mdc-tab{height:48px}.mdc-tab--stacked{height:72px}:host{display:block}.mdc-tab-bar{flex:1}mwc-tab{--mdc-tab-height: 48px;--mdc-tab-stacked-height: 72px}`;let W=class extends K{};W.styles=D,W=l([p("mwc-tab-bar")],W);export{W as TabBar};
