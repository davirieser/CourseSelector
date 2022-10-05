<script lang="ts">
    export let bindto: any;
    export let data: any; 
    export let text = "";
</script>


<select bind:value={bindto} class="select w-full max-w-xs mx-auto flex justify-center">
    <option disabled selected>{text}</option>       
    {#each data.data as data}
      <option>{data.id} {data.name.slice(0, 80)}</option>
    {/each}
</select>

#update