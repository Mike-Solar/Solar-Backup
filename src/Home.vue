<script setup lang="ts">
import { useRouter } from 'vue-router';
const router=useRouter();
const editPlan=()=>{
    router.push("/plan/home");
}
const backup=()=>{
    router.push("/backup/home");
}
const recovery=()=>{
    router.push("/recovery/home");
}
const manage=()=>{
    router.push("/manage/home");
}
</script>
<template>
<h1>欢迎使用 Solar Backup</h1>
<table>
    <tr>
        <td @click="editPlan">
            <img src="assets/plan.svg" class="homelogo"/>
            <br/>
            <span>创建/修改备份计划</span>
        </td>
        <td @click="backup">
            <img src="assets/backup.svg" class="homelogo"/>
            <br/>
            <span>立即备份</span>
        </td>
        <td @click="recovery">
            <img src="assets/recovery.svg" class="homelogo"/>
            <br/>
            <span>还原</span>
        </td>
        <td @click="manage">
            <img src="assets/manage.svg" class="homelogo"/>
            <br/>
            <span>管理备份</span>
        </td>
    </tr>
</table>
</template>
<style>
</style>