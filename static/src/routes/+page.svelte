<script lang="ts">
    import { getLFU, getLFUID } from "./+page";
    import Spinner from "$components/spinner.svelte";
    import Select from '$components/select.svelte';

    let selectedFaculty = "";
    let selectedFacultyID = 0;
    $: selectedFacultyID = parseInt(selectedFaculty.split(" ")[0], 10);

    let selectedCurriculum = "";
    let selectedCurriculumID = 0;
    $: selectedCurriculumID = parseInt(selectedCurriculum.split(" ")[0], 10);

    let selectedCourseCategory = "";
    let selectedCourseCategoryID = 0;
    $: selectedCourseCategoryID = parseInt(selectedCourseCategory.split(" ")[0], 10);

    let tableID = 0;

</script>


<main class="m-5">
    <h1 class="underline text-4xl justify-center mx-0 flex mb-12">Course Selector</h1>
    <div class="grid grid-cols-3 gap-2">
      {#await getLFU()}
        <div class="flex justify-center mx-auto">
          <Spinner />
        </div>
      {:then data}
        <Select bind:bindto = {selectedFaculty} data={data} text="Select Faculty" />
      {:catch error}
        <p>{error}</p>
      {/await}

      
      {#if selectedFaculty.length > 1}
        {#await getLFUID(selectedFacultyID)}
          <div class="flex justify-center mx-auto">
            <Spinner />
          </div>
        {:then data}
          <Select bind:bindto = {selectedCurriculum} data={data} text="Select Curriculum" />
        {:catch error}
          <p>{error}</p>
        {/await}
      {/if}


      {#if selectedCurriculum.length > 1}
        {#await getLFUID(selectedCurriculumID)}
          <div class="flex justify-center mx-auto">
            <Spinner />
          </div>
        {:then data}
          <Select bind:bindto = {selectedCourseCategory} data={data} text="Select Curriculum" />
        {:catch error}
          <p>{error}</p>
        {/await}
      {/if}

      
    </div>  

    <br class="h-5"/>
    <center>
    {#if selectedCourseCategory.length > 1}
        {#await getLFUID(selectedCourseCategoryID)}
          <div class="flex justify-center mx-auto">
            <Spinner />
          </div>
        {:then data}
          <div class="overflow-x-auto">
            <table class="table w-1/2 text-sm">
              <thead>
                <tr>
                  <th></th>
                  <th>ID</th>
                  <th>Name</th>
                  <th></th>
                </tr>
              </thead>
              <tbody>
                {#each data.data as data}
                  <tr class=hover>
                    <th>{tableID++}</th>
                    <td>{data.id}</td>
                    <td>{data.name.slice(0, 80)}</td>
                    <td></td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {:catch error}
          <p>{error}</p>
        {/await}
      {/if}
    </center>
</main> 
