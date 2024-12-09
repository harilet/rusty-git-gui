<script lang="ts">
  export let diff: string;

  $: oldChanges = [""];
  $: newChanges = [""];

  $: {
    let diffAsLines = splitLines();
    if (diff != "") {
      for (var i = 0; i < diffAsLines.length; i++) {
        let change = diffAsLines[i];

        if (startCharIsAdd(change)) {
          var addLineList = change.split(" ");
          addLineList.splice(1, 1);
          var addLine = addLineList.join(" ");

          oldChanges = [...oldChanges, " "];
          newChanges = [...newChanges, addLine];
        } else if (startCharIsSubstract(change)) {
          var substractLineList = change.split(" ");
          substractLineList.splice(2, 1);
          var substractLine = substractLineList.join(" ");

          oldChanges = [...oldChanges, substractLine];
          if (startCharIsAdd(diffAsLines[i + 1])) {
            var substracAddLineList = diffAsLines[i + 1].split(" ");
            substracAddLineList.splice(1, 1);
            var substracAddLine = substracAddLineList.join(" ");

            newChanges = [...newChanges, substracAddLine];
            i = i + 1;
          } else {
            newChanges = [...newChanges, " "];
          }
        } else if (startCharIsAt(change)) {
          oldChanges = [...oldChanges, change, " ", " "];
          newChanges = [...newChanges, " ", " ", " "];
        } else if (startCharIsBlank(change)) {
          var oldLineList = change.split(" ");
          oldLineList.splice(3, 1);
          var oldLine = oldLineList.join(" ");

          var newLineList = change.split(" ");
          newLineList.splice(2, 1);
          var newLine = newLineList.join(" ");

          oldChanges = [...oldChanges, oldLine];
          newChanges = [...newChanges, newLine];
        }
        if (startCharIsAt(diffAsLines[i + 1])) {
          oldChanges = [...oldChanges, " ", " "];
          newChanges = [...newChanges, " ", " "];
        }
        if(i==diffAsLines.length-1){
          oldChanges = [...oldChanges, " ", " "];
          newChanges = [...newChanges, " ", " "];
        }
      }
    } else {
      oldChanges = [""];
      newChanges = [""];
    }
  }

  function splitLines() {
    return diff.split("\n");
  }

  function startCharIsAdd(line: string) {
    return line[0] == "+";
  }

  function startCharIsSubstract(line: string) {
    return line[0] == "-";
  }

  function startCharIsBlank(line: string) {
    return line[0] == " ";
  }

  function startCharIsAt(line: string) {
    return line.slice(0,4) == " _ _";
  }
</script>

<div class="main-file-change-area">
  <div class="old-change">
    {#each oldChanges as line}
      <div class="lines">
        {line}
      </div>
    {/each}
  </div>
  <div class="new-change">
    {#each newChanges as line}
      <div class="lines">
        {line}
      </div>
    {/each}
  </div>
</div>

<style>
  .main-file-change-area {
    display: flex;
    flex-direction: row;
  }

  .lines {
    white-space: pre;
  }

  .lines:hover {
    background: #ffffff40;
  }

  .old-change {
    width: 50%;
  }

  .new-change {
    width: 50%;
  }
</style>
