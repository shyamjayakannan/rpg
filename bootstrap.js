/******/ (function(modules) { // webpackBootstrap
/******/ 	// install a JSONP callback for chunk loading
/******/ 	function webpackJsonpCallback(data) {
/******/ 		var chunkIds = data[0];
/******/ 		var moreModules = data[1];
/******/
/******/
/******/ 		// add "moreModules" to the modules object,
/******/ 		// then flag all "chunkIds" as loaded and fire callback
/******/ 		var moduleId, chunkId, i = 0, resolves = [];
/******/ 		for(;i < chunkIds.length; i++) {
/******/ 			chunkId = chunkIds[i];
/******/ 			if(Object.prototype.hasOwnProperty.call(installedChunks, chunkId) && installedChunks[chunkId]) {
/******/ 				resolves.push(installedChunks[chunkId][0]);
/******/ 			}
/******/ 			installedChunks[chunkId] = 0;
/******/ 		}
/******/ 		for(moduleId in moreModules) {
/******/ 			if(Object.prototype.hasOwnProperty.call(moreModules, moduleId)) {
/******/ 				modules[moduleId] = moreModules[moduleId];
/******/ 			}
/******/ 		}
/******/ 		if(parentJsonpFunction) parentJsonpFunction(data);
/******/
/******/ 		while(resolves.length) {
/******/ 			resolves.shift()();
/******/ 		}
/******/
/******/ 	};
/******/
/******/
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// object to store loaded and loading chunks
/******/ 	// undefined = chunk not loaded, null = chunk preloaded/prefetched
/******/ 	// Promise = chunk loading, 0 = chunk loaded
/******/ 	var installedChunks = {
/******/ 		"main": 0
/******/ 	};
/******/
/******/
/******/
/******/ 	// script path function
/******/ 	function jsonpScriptSrc(chunkId) {
/******/ 		return __webpack_require__.p + "" + chunkId + ".bootstrap.js"
/******/ 	}
/******/
/******/ 	// object to store loaded and loading wasm modules
/******/ 	var installedWasmModules = {};
/******/
/******/ 	function promiseResolve() { return Promise.resolve(); }
/******/
/******/ 	var wasmImportObjects = {
/******/ 		"../pkg/wasm_hello_bg.wasm": function() {
/******/ 			return {
/******/ 				"./wasm_hello_bg.js": {
/******/ 					"__wbindgen_object_drop_ref": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbindgen_object_drop_ref"](p0i32);
/******/ 					},
/******/ 					"__wbg_emitevent_97ffd53c2ab2723c": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_emitevent_97ffd53c2ab2723c"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbindgen_error_new": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbindgen_error_new"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_string_get": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbindgen_string_get"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_string_new": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbindgen_string_new"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_is_object": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbindgen_is_object"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_jsval_loose_eq": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbindgen_jsval_loose_eq"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_boolean_get": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbindgen_boolean_get"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_number_get": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbindgen_number_get"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_new_abda76e883ba8a5f": function() {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_new_abda76e883ba8a5f"]();
/******/ 					},
/******/ 					"__wbg_stack_658279fe44541cf6": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_stack_658279fe44541cf6"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_error_f851667af71bcfc6": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_error_f851667af71bcfc6"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_style_3801009b2339aa94": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_style_3801009b2339aa94"](p0i32);
/******/ 					},
/******/ 					"__wbg_instanceof_CanvasRenderingContext2d_bc0a6635c96eca9b": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_instanceof_CanvasRenderingContext2d_bc0a6635c96eca9b"](p0i32);
/******/ 					},
/******/ 					"__wbg_drawImage_9758fa4c03ab8fc8": function(p0i32,p1i32,p2f64,p3f64) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_drawImage_9758fa4c03ab8fc8"](p0i32,p1i32,p2f64,p3f64);
/******/ 					},
/******/ 					"__wbg_drawImage_f647065c8b6c6c44": function(p0i32,p1i32,p2f64,p3f64,p4f64,p5f64,p6f64,p7f64,p8f64,p9f64) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_drawImage_f647065c8b6c6c44"](p0i32,p1i32,p2f64,p3f64,p4f64,p5f64,p6f64,p7f64,p8f64,p9f64);
/******/ 					},
/******/ 					"__wbg_clearRect_517d3360d8be8a55": function(p0i32,p1f64,p2f64,p3f64,p4f64) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_clearRect_517d3360d8be8a55"](p0i32,p1f64,p2f64,p3f64,p4f64);
/******/ 					},
/******/ 					"__wbg_setsrc_fac5b9516fc69301": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_setsrc_fac5b9516fc69301"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_complete_e123332e1a41aa23": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_complete_e123332e1a41aa23"](p0i32);
/******/ 					},
/******/ 					"__wbg_new_6f9cb260fad32a20": function() {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_new_6f9cb260fad32a20"]();
/******/ 					},
/******/ 					"__wbg_width_2931aaedd21f1fff": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_width_2931aaedd21f1fff"](p0i32);
/******/ 					},
/******/ 					"__wbg_height_0d36fbbeb60b0661": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_height_0d36fbbeb60b0661"](p0i32);
/******/ 					},
/******/ 					"__wbg_getContext_7c5944ea807bf5d3": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_getContext_7c5944ea807bf5d3"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_setProperty_b95ef63ab852879e": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_setProperty_b95ef63ab852879e"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_get_44be0491f933a435": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_get_44be0491f933a435"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_length_fff51ee6522a1a18": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_length_fff51ee6522a1a18"](p0i32);
/******/ 					},
/******/ 					"__wbg_new_898a68150f225f2e": function() {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_new_898a68150f225f2e"]();
/******/ 					},
/******/ 					"__wbindgen_is_function": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbindgen_is_function"](p0i32);
/******/ 					},
/******/ 					"__wbg_next_526fc47e980da008": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_next_526fc47e980da008"](p0i32);
/******/ 					},
/******/ 					"__wbg_next_ddb3312ca1c4e32a": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_next_ddb3312ca1c4e32a"](p0i32);
/******/ 					},
/******/ 					"__wbg_done_5c1f01fb660d73b5": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_done_5c1f01fb660d73b5"](p0i32);
/******/ 					},
/******/ 					"__wbg_value_1695675138684bd5": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_value_1695675138684bd5"](p0i32);
/******/ 					},
/******/ 					"__wbg_iterator_97f0c81209c6c35a": function() {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_iterator_97f0c81209c6c35a"]();
/******/ 					},
/******/ 					"__wbg_get_97b561fb56f034b5": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_get_97b561fb56f034b5"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_call_cb65541d95d71282": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_call_cb65541d95d71282"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_set_502d29070ea18557": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_set_502d29070ea18557"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_isArray_4c24b343cb13cfb1": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_isArray_4c24b343cb13cfb1"](p0i32);
/******/ 					},
/******/ 					"__wbg_instanceof_ArrayBuffer_39ac22089b74fddb": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_instanceof_ArrayBuffer_39ac22089b74fddb"](p0i32);
/******/ 					},
/******/ 					"__wbg_buffer_085ec1f694018c4f": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_buffer_085ec1f694018c4f"](p0i32);
/******/ 					},
/******/ 					"__wbg_new_8125e318e6245eed": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_new_8125e318e6245eed"](p0i32);
/******/ 					},
/******/ 					"__wbg_set_5cf90238115182c3": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_set_5cf90238115182c3"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_length_72e2208bbc0efc61": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_length_72e2208bbc0efc61"](p0i32);
/******/ 					},
/******/ 					"__wbg_instanceof_Uint8Array_d8d9cb2b8e8ac1d4": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbg_instanceof_Uint8Array_d8d9cb2b8e8ac1d4"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_debug_string": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbindgen_debug_string"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_throw": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbindgen_throw"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_memory": function() {
/******/ 						return installedModules["../pkg/wasm_hello_bg.js"].exports["__wbindgen_memory"]();
/******/ 					}
/******/ 				}
/******/ 			};
/******/ 		},
/******/ 	};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId]) {
/******/ 			return installedModules[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/ 	// This file contains only the entry chunk.
/******/ 	// The chunk loading function for additional chunks
/******/ 	__webpack_require__.e = function requireEnsure(chunkId) {
/******/ 		var promises = [];
/******/
/******/
/******/ 		// JSONP chunk loading for javascript
/******/
/******/ 		var installedChunkData = installedChunks[chunkId];
/******/ 		if(installedChunkData !== 0) { // 0 means "already installed".
/******/
/******/ 			// a Promise means "currently loading".
/******/ 			if(installedChunkData) {
/******/ 				promises.push(installedChunkData[2]);
/******/ 			} else {
/******/ 				// setup Promise in chunk cache
/******/ 				var promise = new Promise(function(resolve, reject) {
/******/ 					installedChunkData = installedChunks[chunkId] = [resolve, reject];
/******/ 				});
/******/ 				promises.push(installedChunkData[2] = promise);
/******/
/******/ 				// start chunk loading
/******/ 				var script = document.createElement('script');
/******/ 				var onScriptComplete;
/******/
/******/ 				script.charset = 'utf-8';
/******/ 				script.timeout = 120;
/******/ 				if (__webpack_require__.nc) {
/******/ 					script.setAttribute("nonce", __webpack_require__.nc);
/******/ 				}
/******/ 				script.src = jsonpScriptSrc(chunkId);
/******/
/******/ 				// create error before stack unwound to get useful stacktrace later
/******/ 				var error = new Error();
/******/ 				onScriptComplete = function (event) {
/******/ 					// avoid mem leaks in IE.
/******/ 					script.onerror = script.onload = null;
/******/ 					clearTimeout(timeout);
/******/ 					var chunk = installedChunks[chunkId];
/******/ 					if(chunk !== 0) {
/******/ 						if(chunk) {
/******/ 							var errorType = event && (event.type === 'load' ? 'missing' : event.type);
/******/ 							var realSrc = event && event.target && event.target.src;
/******/ 							error.message = 'Loading chunk ' + chunkId + ' failed.\n(' + errorType + ': ' + realSrc + ')';
/******/ 							error.name = 'ChunkLoadError';
/******/ 							error.type = errorType;
/******/ 							error.request = realSrc;
/******/ 							chunk[1](error);
/******/ 						}
/******/ 						installedChunks[chunkId] = undefined;
/******/ 					}
/******/ 				};
/******/ 				var timeout = setTimeout(function(){
/******/ 					onScriptComplete({ type: 'timeout', target: script });
/******/ 				}, 120000);
/******/ 				script.onerror = script.onload = onScriptComplete;
/******/ 				document.head.appendChild(script);
/******/ 			}
/******/ 		}
/******/
/******/ 		// Fetch + compile chunk loading for webassembly
/******/
/******/ 		var wasmModules = {"0":["../pkg/wasm_hello_bg.wasm"]}[chunkId] || [];
/******/
/******/ 		wasmModules.forEach(function(wasmModuleId) {
/******/ 			var installedWasmModuleData = installedWasmModules[wasmModuleId];
/******/
/******/ 			// a Promise means "currently loading" or "already loaded".
/******/ 			if(installedWasmModuleData)
/******/ 				promises.push(installedWasmModuleData);
/******/ 			else {
/******/ 				var importObject = wasmImportObjects[wasmModuleId]();
/******/ 				var req = fetch(__webpack_require__.p + "" + {"../pkg/wasm_hello_bg.wasm":"126f469c62fa25985539"}[wasmModuleId] + ".module.wasm");
/******/ 				var promise;
/******/ 				if(importObject instanceof Promise && typeof WebAssembly.compileStreaming === 'function') {
/******/ 					promise = Promise.all([WebAssembly.compileStreaming(req), importObject]).then(function(items) {
/******/ 						return WebAssembly.instantiate(items[0], items[1]);
/******/ 					});
/******/ 				} else if(typeof WebAssembly.instantiateStreaming === 'function') {
/******/ 					promise = WebAssembly.instantiateStreaming(req, importObject);
/******/ 				} else {
/******/ 					var bytesPromise = req.then(function(x) { return x.arrayBuffer(); });
/******/ 					promise = bytesPromise.then(function(bytes) {
/******/ 						return WebAssembly.instantiate(bytes, importObject);
/******/ 					});
/******/ 				}
/******/ 				promises.push(installedWasmModules[wasmModuleId] = promise.then(function(res) {
/******/ 					return __webpack_require__.w[wasmModuleId] = (res.instance || res).exports;
/******/ 				}));
/******/ 			}
/******/ 		});
/******/ 		return Promise.all(promises);
/******/ 	};
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function(exports, name, getter) {
/******/ 		if(!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, { enumerable: true, get: getter });
/******/ 		}
/******/ 	};
/******/
/******/ 	// define __esModule on exports
/******/ 	__webpack_require__.r = function(exports) {
/******/ 		if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 			Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 		}
/******/ 		Object.defineProperty(exports, '__esModule', { value: true });
/******/ 	};
/******/
/******/ 	// create a fake namespace object
/******/ 	// mode & 1: value is a module id, require it
/******/ 	// mode & 2: merge all properties of value into the ns
/******/ 	// mode & 4: return value when already ns object
/******/ 	// mode & 8|1: behave like require
/******/ 	__webpack_require__.t = function(value, mode) {
/******/ 		if(mode & 1) value = __webpack_require__(value);
/******/ 		if(mode & 8) return value;
/******/ 		if((mode & 4) && typeof value === 'object' && value && value.__esModule) return value;
/******/ 		var ns = Object.create(null);
/******/ 		__webpack_require__.r(ns);
/******/ 		Object.defineProperty(ns, 'default', { enumerable: true, value: value });
/******/ 		if(mode & 2 && typeof value != 'string') for(var key in value) __webpack_require__.d(ns, key, function(key) { return value[key]; }.bind(null, key));
/******/ 		return ns;
/******/ 	};
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function(module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
/******/ 	};
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function(object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "";
/******/
/******/ 	// on error function for async loading
/******/ 	__webpack_require__.oe = function(err) { console.error(err); throw err; };
/******/
/******/ 	// object with all WebAssembly.instance exports
/******/ 	__webpack_require__.w = {};
/******/
/******/ 	var jsonpArray = window["webpackJsonp"] = window["webpackJsonp"] || [];
/******/ 	var oldJsonpFunction = jsonpArray.push.bind(jsonpArray);
/******/ 	jsonpArray.push = webpackJsonpCallback;
/******/ 	jsonpArray = jsonpArray.slice();
/******/ 	for(var i = 0; i < jsonpArray.length; i++) webpackJsonpCallback(jsonpArray[i]);
/******/ 	var parentJsonpFunction = oldJsonpFunction;
/******/
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = "./bootstrap.js");
/******/ })
/************************************************************************/
/******/ ({

/***/ "./bootstrap.js":
/*!**********************!*\
  !*** ./bootstrap.js ***!
  \**********************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("// A dependency graph that contains any wasm must all be imported\r\n// asynchronously. This `bootstrap.js` file does the single async import, so\r\n// that no one else needs to worry about it again.\r\n(async function() {\r\n  try {\r\n    await __webpack_require__.e(/*! import() */ 0).then(__webpack_require__.bind(null, /*! ./index.js */ \"./index.js\"));\r\n  } catch (error) {\r\n    console.log(error);\r\n  }\r\n})()\r\n// import(\"./index.js\", \"\")\r\n//   .catch(e => console.error(\"Error importing `index.js`:\", e));\r\n\n\n//# sourceURL=webpack:///./bootstrap.js?");

/***/ })

/******/ });