<template>
  <div class="home">
    <h1>DavePomf</h1>
    <div class="caption">Sharing files since 1998</div>
    <captcha>
      <form @submit.prevent="submit" class="upload">
        <div>Upload</div>
        <div class="uploads">
          <div v-for="[file, progress, link] in uploads" :key="link">
            {{ file }} ->
            <span v-if="progress == 100">
              <a :href="link">{{ link }}</a>
            </span>
            <span v-else>{{ progress }}%</span>
          </div>
        </div>
        <div class="upload-button">
          <span>Click here or drop file</span>
          <input ref="upload" type="file" @input="submit" />
        </div>
        <div class="error">{{ error }}</div>
      </form>
    </captcha>
  </div>
</template>

<script>
export default {
  name: "home",
  data: () => ({
    uploads: [],
    error: "",
  }),
  methods: {
    async submit() {
      var formData = new FormData();
      const file = this.$refs.upload.files[0];
      if (file.size > 500 * 1000 * 1000) {
        this.error = "File too big";
        return;
      }
      formData.append("image", file);

      const i = this.uploads.length;
      this.uploads.push([file.name, 0, ""]);

      let source =
        this.$route.query.nocaptcha !== undefined ? "web-nocaptcha" : "web";

      const { data } = await this.http.post(
        `/upload?source=${source}`,
        formData,
        {
          headers: {
            "Content-Type": "multipart/form-data",
          },
          onUploadProgress: (e) => {
            const progress = Math.round((e.loaded / e.total) * 100);

            const [name, prev_prog, url] = this.uploads[i];
            if (prev_prog === 100) return;

            this.uploads[i] = [name, progress, url];
            this.$forceUpdate();
          },
        }
      );

      const [name, _prog, _url] = this.uploads[i];
      this.uploads[i] = [name, 100, data];
      setTimeout(() => {
        this.$forceUpdate();
      }, 100);
    },
  },
};
</script>

<style lang="scss">
.home {
  padding: 2rem;
  background: #fff;
  box-shadow: 0 3px 1px -2px rgba(0, 0, 0, 0.2), 0 2px 2px 0 rgba(0, 0, 0, 0.14),
    0 1px 5px 0 rgba(0, 0, 0, 0.12);
  .captcha {
    padding: 100px;
  }
  .upload {
    padding-top: 2rem;

    .upload-button {
      position: relative;
      margin-top: 1rem;
      font-size: 1.2rem;
      border: solid #4a4a4a4a 1px;
      border-radius: 8px;

      width: 50%;
      height: 128px;
      span {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
      }
      input[type="file"] {
        width: 100%;
        height: 100%;
        position: absolute;
        left: 0;
        top: 0;
        opacity: 0;
      }
    }
    .error {
      padding-top: 1rem;
      color: #ea6962;
      font-size: 1.5rem;
    }
  }
}
</style>