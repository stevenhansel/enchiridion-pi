"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (_) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
var fastify_1 = __importDefault(require("fastify"));
var axios_1 = __importDefault(require("axios"));
var fs_1 = __importDefault(require("fs"));
var path_1 = __importDefault(require("path"));
var PORT = 8080;
var IMAGE_DIRECTORY;
var fastify = (0, fastify_1.default)({ logger: true });
var downloadImage = function (url, imagePath) { return __awaiter(void 0, void 0, void 0, function () {
    return __generator(this, function (_a) {
        return [2 /*return*/, (0, axios_1.default)({ url: url, responseType: 'stream' }).then(function (response) {
                return new Promise(function (resolve, reject) {
                    response.data
                        .pipe(fs_1.default.createWriteStream(imagePath))
                        .on('finish', function () { return resolve(true); })
                        .on('error', function (err) { return reject(err); });
                });
            })];
    });
}); };
/**
 * POST /images
 * JSON: string[]
 *
 * response: boolean
 */
fastify.post('/images', function (req) { return __awaiter(void 0, void 0, void 0, function () {
    var imageUrls, responses, result, errors, _i, result_1, res;
    return __generator(this, function (_a) {
        switch (_a.label) {
            case 0:
                imageUrls = req.body;
                responses = imageUrls.map(function (url, index) {
                    var filename = url.split('/').pop() + '.jpeg' || "image_".concat(index);
                    return downloadImage(url, path_1.default.join(path_1.default.format(IMAGE_DIRECTORY), filename));
                });
                return [4 /*yield*/, Promise.all(responses)];
            case 1:
                result = _a.sent();
                errors = [];
                for (_i = 0, result_1 = result; _i < result_1.length; _i++) {
                    res = result_1[_i];
                    if (res instanceof Error) {
                        errors.push(res);
                        fastify.log.error(errors);
                    }
                }
                return [2 /*return*/, { status: errors.length > 0 ? false : true }];
        }
    });
}); });
var main = function () { return __awaiter(void 0, void 0, void 0, function () {
    var args, address, err_1;
    return __generator(this, function (_a) {
        switch (_a.label) {
            case 0:
                args = process.argv.slice(2);
                if (args.length < 1) {
                    process.exit(1);
                }
                IMAGE_DIRECTORY = path_1.default.parse(args[0]);
                _a.label = 1;
            case 1:
                _a.trys.push([1, 3, , 4]);
                return [4 /*yield*/, fastify.listen(PORT)];
            case 2:
                address = _a.sent();
                fastify.log.info("Server is now starting on ".concat(address));
                return [3 /*break*/, 4];
            case 3:
                err_1 = _a.sent();
                fastify.log.error(err_1);
                process.exit(1);
                return [3 /*break*/, 4];
            case 4: return [2 /*return*/];
        }
    });
}); };
main();
