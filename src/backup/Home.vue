<script setup lang="ts">
import { useRouter } from 'vue-router';
let per:number;
const back=()=>{
    useRouter().back();
}
</script>
<template>
<div id="processBar">
    <div id="completed"></div>
</div>
<span>已完成{{ per }}%</span>
<button @click="back">返回</button>
</template>