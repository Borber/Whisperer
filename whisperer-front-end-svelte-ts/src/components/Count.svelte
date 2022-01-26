<script lang="ts">
    import axios from "axios";

    const data = JSON.stringify({
        "database": {
            "uri_id": "FREE_MONGODB",
            "database": "whisperer",
            "collection": "count"
        }
    });

    const config = {
        method: 'post',
        url: 'https://vercel-mongodb-count.vercel.app/api/v1/count',
        headers: {
            'Content-Type': 'application/json'
        },
        data: data
    };


    let count = 0;
    let init_count = false;
    const sleep = async (ms) => {
        return new Promise((resolve) => setTimeout(resolve, ms ?? 200));
    }

    export const local_add_one = async () => {
        count++
    }

    const show_count = async () => {
        axios(config)
            .then(async (response) => {
                count
                let total = response.data.total;
                init_count = true;
                let current = count;
                let accumulation = Math.floor(Math.abs(total - current) / 200);
                accumulation = accumulation > 1 ? accumulation : 1;
                if (total > current) {
                    for (let i = current; i <= total; i = i + accumulation) {
                        count = i;
                        await sleep(10);
                    }
                    count = total;
                } else {
                    for (let i = current; i >= total; i = i - accumulation) {
                        count = i;
                        await sleep(10);
                    }
                    count = total;
                }
            })
            .catch(function (error) {
                console.log(error);
            });
        while (!init_count) {
            count = count + 1
            await sleep(10);
        }
    }

    window.onload = async () => {
        await show_count()
    }
</script>

<main>
    <p class="count_doc">API共计被调用 <i class="count">{count}</i> 次</p>
</main>

<style global lang="scss">
  .count_doc {
    color: #a7a9be;
    text-align: center;
    font-weight: bold;
    font-family: "FOT-ModeMinALarge Std", serif;
  }

  .count {
    color: #6fd287;
  }
</style>