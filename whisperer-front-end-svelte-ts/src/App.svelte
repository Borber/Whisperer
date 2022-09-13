<script lang="ts">
    import axios from "axios";
    import Count from "./components/Count.svelte";

    let encode_text: string;
    let decode_text: string;
    let count_c: Count;
    let result = "";
    let github_icon: string;
    let github_star: string;

    const en = () => {
        axios.post("/v1/api/e", {
            s: encode_text
        }).then(async (response) => {
            result = response.data.result;
            decode_text = result
            await count_c.local_add_one()
        })
    };
    const de = () => {
        axios.post("/v1/api/d", {
            s: decode_text
        }).then(async (response) => {
            result = response.data.result;
            encode_text = result
            await count_c.local_add_one()
        })
    };
    const cp = () => {
        navigator.clipboard.writeText(result);
    }
    const clear = () => {
        encode_text = ""
        decode_text = ""
    }

    window.onload = async () => {
        github_icon = "https://img.shields.io/static/v1?&label=&message=Whisper"
        github_star = "https://img.shields.io/github/stars/Borber/Whisperer.svg?style=social"
        await count_c.show_count()
    }
</script>

<main>
    <div class="container column is-4 is-offset-4">
        <h1 class="whisperer">Whisperer / 低语者</h1>
        <textarea bind:value={encode_text} class="textarea fot has-fixed-size block" placeholder="明文"></textarea>
        <div class="block">
            <button class="button is-info fot" id="eb" on:click={en}>
                低语
            </button>
            <button class="button is-success fot" id="db" on:click={de}>
                高歌
            </button>
            <button class="button" on:click={cp}>
                𝑪𝑶𝑷𝒀
            </button>
            <button class="button is-warning is-light" on:click={clear}>
                𝑪𝑳𝑬𝑨𝑹
            </button>
        </div>
        <textarea bind:value={decode_text} class="textarea fot has-fixed-size block" placeholder="密文"></textarea>
        <p class="announcement fot">⬥ 如果您喜欢本项目并能给一个<i class="star"> star </i>就太好了 <a
                href="https://github.com/Borber/Whisperer" target="_blank"><img
                alt="Whisperer" src={github_icon}/></a> <img alt="stars" src={github_star}>
        </p>
        <p class="announcement fot">⬥ 本项目后端完全开源, 无法记录任何加密内容, 请放心使用。</p>
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
    border-radius: 0 !important;
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

  .star {
    color: #6fd287;
  }
</style>
