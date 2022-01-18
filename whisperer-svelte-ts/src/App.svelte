<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri";

    let encode_text;
    let result = "";
    const en = () => {
        invoke("encode_api", {
            endpoint: "加密",
            body: {
                s: (<HTMLInputElement> encode_text).value
            },
        }).then(
            (res) => {
                result = res.toString()
            }
        )
    };
    const de = () => {
        invoke("decode_api", {
            endpoint: "解密",
            body: {
                s: (<HTMLInputElement>encode_text).value
            },
        }).then(
            (res) => {
                result = res.toString()
            }
        )
    };
    const cp = () => {
        navigator.clipboard.writeText(result);
    }
</script>

<textarea bind:this={encode_text}></textarea>

<button on:click={en}>
    加密
</button>
<button on:click={de}>
    解密
</button>

<p>{result}</p>

<button on:click={cp}>
    复制
</button>

<style>
</style>
