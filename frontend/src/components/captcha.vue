<template>
  <div class="captcha-wrapper">
    <div class="captcha" v-if="!cleared">
      <h1>Captcha</h1>
      <div class="caption" @mousedown="prevent" @select="prevent">
        Vilken bokstav representerar: {{ question }}
      </div>
      <form @submit.prevent="check">
        <input v-model="input" />
      </form>
    </div>
    <div v-else>
      <slot />
    </div>
  </div>
</template>

<script>
export default {
  name: "captcha",
  data: () => ({
    question: "",
    answer: "",
    input: "",
    cleared: false,
  }),
  mounted() {
    const [question, answer] = this.get_captcha();
    this.question = question;
    this.answer = answer;
    if (this.$route.query.nocaptcha !== undefined) {
      this.cleared = true;
    }
  },
  methods: {
    get_captcha() {
      const questions = [
        ["Centerpartiet", "C"],
        ["Kristdemokraterna", "KD"],
        ["Liberalerna", "L"],
        ["Miljöpartiet", "MP"],
        ["Moderaterna", "M"],
        ["Socialdemokraterna", "S"],
        ["Sverigedemokraterna", "SD"],
        ["Vänsterpartiet", "V"],
      ];
      return questions[this.getRandomInt(0, questions.length)];
    },
    check() {
      if (this.answer.toLowerCase() == this.input.toLowerCase()) {
        this.cleared = true;
      }
    },
    getRandomInt(min, max) {
      min = Math.ceil(min);
      max = Math.floor(max);
      return Math.floor(Math.random() * (max - min)) + min;
    },
    prevent(e) {
      e.preventDefault();
    },
  },
};
</script>

<style lang="scss">
.captcha {
  padding: 1rem;
  .question {
    font-size: 1.2rem;
  }
}
</style>