<script setup lang="ts">
const selectSource=()=>{

}
const selectToLocal=()=>{

}
const selectToNet=()=>{

}
const selectSaveWeek=()=>{

}
const selectSaveMonth=()=>{

}
const selectSaveYear=()=>{

}
const selectSaveFar=()=>{

}
const save=()=>{
    
}
</script>
<template>
<h1>创建/修改备份计划</h1>
<table>
    <tr>
        <td>要备份的文件夹：</td>
        <td><button @click="selectSource">选择</button></td>
    </tr>
    <tr>
        <td>备份到：</td>
        <td>
            <label for="backupToLocal">保存在本地</label>
            <input type="radio" id="backupToLcal" name="backupTo" />
            <button @click="selectToLocal">选择</button>
            <br/>
            <label for="backupToNetViaSMB">保存在SMB网络位置</label>
            <input type="radio" id="backupToNetViaSMB" name="backupTo" />
            <button @click="selectToNet">选择</button>
        </td>
    </tr>
    <tr>
        <td>备份保存计划：</td>
        <td>
            <table>
                <tr>
                    <td>保留最近7天的备份：</td>
                    <td>
                        <select @change="selectSaveWeek">
                            <option value="everyday">每天</option>
                            <option value="everyhour">每小时</option>
                        </select>
                    </td>
                </tr>
                <tr>
                    <td>保留最近一个月的备份：</td>
                    <td>
                        <select @change="selectSaveMonth">
                            <option value="everyweek">每周</option>
                            <option value="everyday">每天</option>
                            <option value="everyhour">每小时</option>
                        </select>
                    </td>
                </tr>
                <tr>
                    <td>保留最近一年的备份：</td>
                    <td>
                        <select @change="selectSaveYear">                       
                            <option value="everymonth">每月</option>
                            <option value="everyweek">每周</option>
                            <option value="everyday">每天</option>
                            <option value="everyhour">每小时</option>
                        </select>
                    </td>
                </tr>
                <tr>
                    <td>保留一年前的备份：</td>
                    <td>
                        <select @change="selectSaveFar">
                            <option value="everyyear">每年</option>
                            <option value="everymonth">每月</option>
                            <option value="everyweek">每周</option>
                            <option value="everyday">每天</option>
                            <option value="everyhour">每小时</option>
                        </select>
                    </td>
                </tr>
            </table>
        </td>
    </tr>
</table>
<button @click="save">保存</button>
</template>