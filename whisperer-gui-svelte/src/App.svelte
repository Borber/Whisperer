<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    let encode_text: string;
    let decode_text: string;
    let result: string;

    const en = () => {
        console.log(encode_text);
        if(encode_text){
            invoke("encode_api", {
                endpoint: "加密",
                body: {
                    s: encode_text,
                },
            }).then((res) => {
                result = res.toString();
                decode_text = result;
                navigator.clipboard.writeText(result);
            });
        } else {
            decode_text=""
        }
        
    };
    const de = () => {
        invoke("decode_api", {
            endpoint: "解密",
            body: {
                s: decode_text,
            },
        }).then((res) => {
            result = res.toString();
            encode_text = result;
        });
    };

</script>

<textarea bind:value={encode_text} on:input={en} id="et" placeholder="明文" />

<textarea bind:value={decode_text} on:input={de} id="dt" placeholder="密文" />
<style>
    :global(body) {
        margin: 0;
    }

    :global(#app) {
        display: flex;
        justify-content: space-between;
    }
    #et{
        border-right: 1px solid #3da9fc;
    }

    textarea {
        width: 50%;
        height: 100%;
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

</style>
