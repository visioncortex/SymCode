export default {
    beautifyJSON: (json) => {
        return JSON.stringify(json, null, 2);
    }
};