<script lang="ts">
  let { size = 96 }: { size?: number } = $props();
</script>

<!-- SVG uses currentColor so CSS color prop tints it -->
<svg
  viewBox="0 0 512 512"
  width={size}
  height={size}
  xmlns="http://www.w3.org/2000/svg"
  aria-hidden="true"
>
  <line stroke-width="14.4" stroke-linecap="round"
    x1="220" y1="193.6" x2="200" y2="158.8"
    stroke="currentColor" fill="none" />
  <line stroke-width="14.4" stroke-linecap="round"
    x1="318.4" y1="292" x2="353.2" y2="312"
    stroke="currentColor" fill="none" />
  <line stroke-width="14.4" stroke-linecap="round"
    x1="220" y1="318.4" x2="200" y2="353.2"
    stroke="currentColor" fill="none" />
  <circle cx="256" cy="256" r="72" fill="currentColor" />
  <circle cx="173" cy="112" r="54" fill="currentColor" />
  <circle cx="400" cy="339" r="54" fill="currentColor" />
  <circle cx="173" cy="400" r="54" fill="currentColor" />
</svg>
