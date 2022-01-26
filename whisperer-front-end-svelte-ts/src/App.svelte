<script lang="ts">
    import axios from "axios";
    import Count from "./components/Count.svelte";

    let encode_text;
    let decode_text;
    let count_c;
    let result = "";

    const en = () => {
        axios.post("https://whisperer-serverless-vercel.vercel.app/api/v1/encode", {
            s: encode_text.value
        }).then(async (response) => {
            result = response.data.result;
            decode_text.value = result
            await count_c.local_add_one()
        })
    };
    const de = () => {
        axios.post("https://whisperer-serverless-vercel.vercel.app/api/v1/decode", {
            s: decode_text.value
        }).then(async (response) => {
            result = response.data.result;
            encode_text.value = result
            await count_c.local_add_one()
        })
    };
    const cp = () => {
        navigator.clipboard.writeText(result);
    }
</script>

<main>
    <div class="container column is-4 is-offset-4">
        <h1 class="whisperer">Whisperer</h1>
        <textarea bind:this={encode_text} class="textarea fot has-fixed-size block" placeholder="明文"></textarea>
        <div class="block">
            <button class="button is-info fot" id="eb" on:click={en}>
                低语
            </button>
            <button class="button is-success fot" id="db" on:click={de}>
                高歌
            </button>
            <button class="button fot" id="copy" on:click={cp}>
                𝑪𝑶𝑷𝒀
            </button>
        </div>
        <textarea bind:this={decode_text} class="textarea fot has-fixed-size block" placeholder="密文"></textarea>
        <p class="announcement fot">⬥ 如果您喜欢本项目并能给一个star就太好了 <a href="https://github.com/Borber/Whisperer"
                                                               target="_blank"><img
                alt="Whisperer" src="https://img.shields.io/static/v1?&label=&message=Whisper"/></a> <img alt="stars"
                                                                                                          src="https://img.shields.io/github/stars/Borber/Whisperer.svg?style=social">
        </p>
        <p class="announcement fot">⬥ Copy: 复制最新生成的值</p>
        <p class="announcement fot">⬥ 推荐字体：<a class="hlink" href="https://mp.weixin.qq.com/s/zkV_yDuJalN62PqcdEcsHQ">BC咒术回战</a>
        </p>
        <Count bind:this={count_c}/>
    </div>
</main>

<style global lang="scss">

  // Set your brand colors
  $pink: #FA7C91;
  $link: $pink;

  // Import only what you need from Bulma
  @import "../node_modules/bulma/sass/utilities/_all.sass";
  @import "../node_modules/bulma/sass/elements/button.sass";
  @import "../node_modules/bulma/sass/form/shared.sass";
  @import "../node_modules/bulma/sass/form/input-textarea.sass";
  @import "../node_modules/bulma/sass/layout/_all.sass";
  @import "../node_modules/bulma/sass/grid/columns.sass";

  .block:not(:last-child) {
    margin-bottom: 1.5rem;
  }

  button {
    font-weight: bold;
  }

  button:not(:last-child) {
    margin-right: 0.5rem;
  }

  .whisperer {
    color: #a7a9be;
  }

  .textarea {
    border-radius: 0;
    background: #0f0e17;
    color: #a7a9be;
  }

  .textarea::placeholder {
    color: #a7a9be;
    font-family: "FOT-ModeMinALarge Std", serif;
  }

  body {
    background-image: url("static/christmas-black.png");
    height: 100vh;
    margin: 0;
  }

  .announcement {
    color: #a7a9be;
    font-weight: bold;
  }

  .fot {
    font-family: "FOT-ModeMinALarge Std", serif;
  }

  .hlink {
    color: #6fd287;
  }
</style>
