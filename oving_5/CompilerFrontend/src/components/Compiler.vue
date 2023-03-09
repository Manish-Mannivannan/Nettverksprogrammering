<script setup lang="ts">
  import { ref } from "vue";
  import axios from "axios";

  let input = ref('fn main(){println!("Hello World");}');
  let output = ref("");

  async function compile() {
    console.log(input.value);
    axios.post("http://127.0.0.1:8080", input.value)
      .then((response) => {
        output.value = response.data;
      })
      .catch((error) => {
        console.log(error);
      })
  }
</script>

<template>
  <div class="container">
    <div>
      <textarea name="input" id="input" cols="60" rows="20" class="input" v-model="input" placeholder="Input Rust code to be compiled"></textarea>
      <textarea name="output" id="output"  rows="20" class="output" disabled>{{ output }}</textarea>
    </div>
    <button type="button" class="compile" @click="compile()">Compile</button>
  </div>
</template>

<style scoped>
  div{
    display: flex;
    flex-flow: row wrap;
    justify-content: center;
    width: 100%;    
  }

  textarea{
    margin: .5em;
  }

  #output{
    flex: 1;
  }

  .compile{
    margin: 1em;
  }
</style>
