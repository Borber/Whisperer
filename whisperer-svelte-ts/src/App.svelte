<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri";

    let encode_text: string;
    let decode_text: string;
    let result = "";
    const en = () => {
        invoke("encode_api", {
            endpoint: "加密",
            body: {
                s: encode_text
            },
        }).then(
            (res) => {
                result = res.toString()
                decode_text = result
            }
        )
    };
    const de = () => {
        invoke("decode_api", {
            endpoint: "解密",
            body: {
                s: decode_text
            },
        }).then(
            (res) => {
                result = res.toString()
                encode_text = result
            }
        )
    };
    const cp = () => {
        navigator.clipboard.writeText(result);
    }
</script>

<textarea bind:value={encode_text} id="et" placeholder="明文"></textarea>

<div class="buttons">
    <button id="eb" on:click={en}>
        ▶
    </button>
    <button id="db" on:click={de}>
        ◀
    </button>
    <button id="copy" on:click={cp}>
        𝑪𝑶𝑷𝒀
    </button>
</div>

<textarea bind:value={decode_text} id="dt" placeholder="密文"></textarea>

<style>
    :global(body) {
        margin: 0;
    }

    :global(#app) {
        display: flex;
    }

    textarea {
        width: 262px;
        height: 320px;
        border: none;
        box-sizing: border-box;
        color: #a7a9be;
        background: #0f0e17;
        padding: 0.5em;
        font-size: 1em;
        font-family: "FOT-ModeMinALarge Std", serif;
        resize: none;
    }

    textarea:focus {
        outline: none;
    }

    textarea::-webkit-scrollbar {
        width: 6px;
        height: 6px;
    }

    textarea::-webkit-scrollbar-thumb {
        border-radius: 3px;
        -moz-border-radius: 3px;
        -webkit-border-radius: 3px;
        background-color: #c3c3c3;
    }

    textarea::-webkit-scrollbar-track {
        background-color: transparent;
    }

    button {
        padding: 0;
        border: none;
        color: #fffffe;
        width: 76px;
        height: 106px;
    }

    .buttons {
        width: 76px;
    }

    #eb {
        background: #ff8905;
    }

    #db {
        background: #f25f4c;
    }

    #copy {
        height: 108px;
        background: #3da9fc;
    }

</style>
