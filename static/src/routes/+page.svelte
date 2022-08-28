<script lang="ts">
    export let data: any; 

</script>


<main class="">
    <h1 class="underline text-4xl justify-center mx-0 flex mb-12">Course Selector</h1>
    <div class="grid grid-cols-4 gap-4 m-10">
    {#each data.data.data as faculty}
        <div class="card text-sm w-auto bg-base-100 shadow-xl m-2">
            <div class="card-body text-sm justify-center">
              <h2 class="card-title text-sm justify-center">{faculty.id} <br/>{faculty.name} </h2>
              <div class="card-actions justify-center">
                <button class="btn btn-primary">See courses</button>
              </div>
            </div>
          </div>
         
    {/each}
    </div> 
</main> 
