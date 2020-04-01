<template>
  <div id="ping">
    <router-view></router-view>
    <p>{{pong}}</p>
  </div>
</template>

<script>
export default {
  data() {
    return {
      pong: []
    };
  },

  created: function() {
    this.ping();
  },

  methods: {
    ping() {
      let uri = "api/ping";
      console.log(uri);
      this.axios
        .get(uri)
        .then(response => {
          this.pong = response.data;
        })
        .catch(err => {
          console.log("Error => " + err);
        });
    }
  }
};
</script>

<style>
</style>