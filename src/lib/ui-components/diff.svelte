<script lang="ts">
  export let diff: any[];

  function startCharIsAdd(line: any) {
    return line['change_type'] == "+";
  }

  function startCharIsSubstract(line: any) {
    return line['change_type'] == "-";
  }

  function startCharIsAt(line: string) {
    return line.slice(0, 4) == " _ _";
  }

  function formatString(diff: any) {
    return diff['content'];
  }

  function lineClass(line: string){
    if(startCharIsAdd(line)){
      return 'change-add';
    }else if (startCharIsSubstract(line)){
      return 'change-remove';
    }
    return '';
  }

  function getFromLineNo(line: any){
    return line['from_no'];
  }

  function getToLineNo(line: any){
    return line['to_no'];
  }
</script>

<div class="main-file-change-area">
  <div class="overflow-auto-style w-full">
    <table class="w-full">
      <tbody>
      {#each diff as line}
      <tr class="lines {lineClass(line)} flex">
        <td>
          <div class="from-change-line-no">{getFromLineNo(line)}</div>
        </td>
        <td>
          <div class="to-change-line-no">{getToLineNo(line)}</div>
        </td>
        <td>
          <div class="main-text">{formatString(line)}</div>
        </td>
      </tr>
      {/each}
    </tbody>
    </table>
  </div>
</div>

<style>
  .main-file-change-area {
    display: flex;
    flex-direction: row;
  }

  .lines {
    white-space: pre;
    margin: 3px;
    padding: 2px;
    border-radius: 2px;
  }

  .lines:hover {
    background: #ffffff40;
  }

  .change-add {
    background: #1a7f2266;
  }

  .change-remove {
    background: #7f1a1a66;
  }

  .from-change-line-no{
    padding: 1px;
    margin: 2px;
    width: 25px;
    white-space: pre;
    margin-right: 5px;
  }

  .to-change-line-no{
    white-space: pre;
    padding: 1px;
    margin: 2px;
    width: 25px;
    float: right;
    text-align: right;
    padding-right: 0px;
    margin-right: 0px;
    margin-left: 5px;
  }

  .main-text{
    padding: 1px;
    margin: 2px;
    padding-left: 0px;
    margin-left: 0px;
  }
</style>
