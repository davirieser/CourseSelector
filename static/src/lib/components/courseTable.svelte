<script lang="ts">
	import Spinner from '$components/spinner.svelte';
    import { getCourse } from "$src/routes/+page";
    import { selectedCoursesStore } from "$lib/stores/selectedCourses";

    export let selectedCourseVarationID = 0;
    export let selectedCourseName = "";

    let tempGroupNumber = 0;
    // let once = false;

    function handleSelect(groupData: any){
        console.log("Adding to storage array");
        let selectedCourses = $selectedCoursesStore;
        let alreadyInStore = false;
        selectedCourses.forEach(group => {
            if(group.number == groupData.number){
                console.log("Group already exists");
                alreadyInStore=true;
                return;
            }
        });
        if(alreadyInStore) throw new Error("Group already exists");
        $selectedCoursesStore = [...$selectedCoursesStore, {
			group: groupData,
		}];
    }
</script>

{#await getCourse(selectedCourseVarationID, selectedCourseName)}
    <div class="flex justify-center mx-auto">
        <Spinner />
    </div>
{:then data} 
    <div class="w-auto mx-auto justify-center flex">
        <table class="table w-auto mx-auto">
            <thead>
                <tr>
                    <th>Group</th>
                    <th>Date</th>
                    <th>Time</th>
                    <th>Location</th>
                    <th>Comment</th>
                    <th>Add to calendar</th>
                </tr>
            </thead>
            <tbody>
                {#each data.groups as group}
                    {#each group.times as times}     
                        <tr>
                            <th>{group.number}</th>
                            <td>{times.date}</td>
                            <td>{times.time}</td>
                            <td>{times.location}</td>
                            <td>{times.comment}</td>
                            {#if group.number !== tempGroupNumber && group.number !== 0}
                                    <td><button on:click={handleSelect(group)} class="btn btn-primary">Add Group {tempGroupNumber = group.number} {selectedCourseName.slice(0,2)} to calendar</button></td>
                                {:else if group.number === 0}
                                    <td><button class="btn btn-primary">Add Group {data.number} {selectedCourseName.slice(0,2)} to calendar</button></td>
                                    <!-- {once = true} -->
                                {:else}
                                    <td></td>
                            {/if}
                        </tr>
                    {/each}
                    <tr>
                        <th>---</th>
                        <td>---</td>
                        <td>---</td>
                        <td>---</td>
                        <td>---</td>
                        <td>---</td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
{/await}