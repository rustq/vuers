const {
    h,
    withDirectives,
    ref,
    watch,
    onMounted,
    onBeforeUnmount,
    onUpdated,
    getCurrentInstance,
    reactive,
} = require('vue');

export default {
    props: {
        config: {
            type: Object,
            default: function () {
                return {};
            },
        },
        __useStrictMode: {
            type: Boolean,
        },
    },

    inheritAttrs: false,

    setup(props: any, {attrs, slots, expose, emits}: any) {

        const container = ref(null);
        const instance = getCurrentInstance();
        // @ts-ignore
        const ssw = global.ssw;
        const core = new ssw.SoftSkiaWASM();
        instance.ssw = core; // Save on component instance
        console.log(core)
        core.setShapeToChild(0, 0, 0, attrs.width, attrs.height, 0, 0, 0, 0)


        onMounted(() => {
            const base64 = core.toBase64();
            container.value.setAttribute("src", base64);
        });

        onUpdated(() => {
            const base64 = core.toBase64();
            console.log(core.toDebug())
            container.value.setAttribute("src", base64);
        });

        onBeforeUnmount(() => {
        });


        return () => h('img', { ref: container }, slots.default?.());
    }
}