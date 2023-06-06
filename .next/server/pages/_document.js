"use strict";
(() => {
var exports = {};
exports.id = 660;
exports.ids = [660];
exports.modules = {

/***/ 7669:
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

// ESM COMPAT FLAG
__webpack_require__.r(__webpack_exports__);

// EXPORTS
__webpack_require__.d(__webpack_exports__, {
  "default": () => (/* reexport */ Document)
});

// EXTERNAL MODULE: ./node_modules/react/jsx-runtime.js
var jsx_runtime = __webpack_require__(5893);
// EXTERNAL MODULE: ./node_modules/next/document.js
var next_document = __webpack_require__(6859);
;// CONCATENATED MODULE: ./pages/_document.js


function Document() {
    const meta = {
        title: "Next.js Blog Starter Kit",
        description: "Clone and deploy your own Next.js portfolio in minutes.",
        image: "https://assets.vercel.com/image/upload/q_auto/front/vercel/dps.png"
    };
    return /*#__PURE__*/ (0,jsx_runtime.jsxs)(next_document.Html, {
        lang: "en",
        children: [
            /*#__PURE__*/ (0,jsx_runtime.jsxs)(next_document.Head, {
                children: [
                    /*#__PURE__*/ jsx_runtime.jsx("meta", {
                        name: "robots",
                        content: "follow, index"
                    }),
                    /*#__PURE__*/ jsx_runtime.jsx("meta", {
                        name: "description",
                        content: meta.description
                    }),
                    /*#__PURE__*/ jsx_runtime.jsx("meta", {
                        property: "og:site_name",
                        content: meta.title
                    }),
                    /*#__PURE__*/ jsx_runtime.jsx("meta", {
                        property: "og:description",
                        content: meta.description
                    }),
                    /*#__PURE__*/ jsx_runtime.jsx("meta", {
                        property: "og:title",
                        content: meta.title
                    }),
                    /*#__PURE__*/ jsx_runtime.jsx("meta", {
                        property: "og:image",
                        content: meta.image
                    })
                ]
            }),
            /*#__PURE__*/ (0,jsx_runtime.jsxs)("body", {
                children: [
                    /*#__PURE__*/ jsx_runtime.jsx(next_document.Main, {}),
                    /*#__PURE__*/ jsx_runtime.jsx(next_document.NextScript, {})
                ]
            })
        ]
    });
}

;// CONCATENATED MODULE: ./node_modules/next/dist/build/webpack/loaders/next-route-loader.js?page=%2F_document&absolutePagePath=private-next-pages%2F_document.js&preferredRegion=!

        // Next.js Route Loader
        
        
    

/***/ }),

/***/ 4140:
/***/ ((module) => {

module.exports = require("next/dist/server/get-page-files.js");

/***/ }),

/***/ 9716:
/***/ ((module) => {

module.exports = require("next/dist/server/htmlescape.js");

/***/ }),

/***/ 6368:
/***/ ((module) => {

module.exports = require("next/dist/server/utils.js");

/***/ }),

/***/ 6724:
/***/ ((module) => {

module.exports = require("next/dist/shared/lib/constants.js");

/***/ }),

/***/ 8743:
/***/ ((module) => {

module.exports = require("next/dist/shared/lib/html-context.js");

/***/ }),

/***/ 8524:
/***/ ((module) => {

module.exports = require("next/dist/shared/lib/is-plain-object.js");

/***/ }),

/***/ 6689:
/***/ ((module) => {

module.exports = require("react");

/***/ })

};
;

// load runtime
var __webpack_require__ = require("../webpack-runtime.js");
__webpack_require__.C(exports);
var __webpack_exec__ = (moduleId) => (__webpack_require__(__webpack_require__.s = moduleId))
var __webpack_exports__ = __webpack_require__.X(0, [893,859], () => (__webpack_exec__(7669)));
module.exports = __webpack_exports__;

})();