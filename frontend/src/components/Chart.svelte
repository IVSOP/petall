<script>
  import { onMount, onDestroy } from 'svelte';

  // try using zeroes!!!
  export let y_values = [-10, -5, 1, 5, 10, 3, -2, -7, 4, 8];
  export let isTimeseries = false;
  const labelName = "SOMENAME";

  let chart;
  let chartEl;

  const raw_points = y_values.map((y, index) => { return {x: index, y: y} });

  function getZeroX(pointA, pointB) {
      let m = (pointB.y - pointA.y) / (pointB.x - pointA.x);
      let b = pointA.y - (m * pointA.x);
      return - (b / m);
    }
    
  const green = '#00c853';
  const red = '#d50000';
  function processPoints(points_in, colors, points_out, offset) {
    // console.log(`Processing from ${offset} to ${points_in.length}`);

    if (points_in.length > 0) {
        
        let first = points_in[offset];
        let res = [first]
        // console.log(`first is ${first.x, first.y}`);
        if (first.y >= 0) {
            colors.push(green);
            for (let i = offset + 1; i < points_in.length; i++) {
                let point = points_in[i];
                if (point.y >= 0) {
                    res.push(point);
                } else {
                    let intersect_x = getZeroX(points_in[i - 1], points_in[i]);
                    let intersect = {x: intersect_x, y: -0.00000000001};
                    res.push(intersect);
                    points_in[i - 1] = intersect;
                    // console.log(i);
                    processPoints(points_in, colors, points_out, i - 1);
                    break;
                }
            }
        } else {
            colors.push(red);
            for (let i = offset + 1; i < points_in.length; i++) {
                let point = points_in[i];
                if (point.y >= 0) {
                    let intersect_x = getZeroX(points_in[i - 1], points_in[i]);
                    let intersect = {x: intersect_x, y: 0.00000000001};
                    res.push(intersect);
                    points_in[i - 1] = intersect;
                    // console.log(i);
                    processPoints(points_in, colors, points_out, i - 1);
                    break;
                } else {
                    res.push(point);
                }
            }
        }
        
        points_out.push(res);
    }
  }

  // dont forget to invert points_out!!!!!!!
  let processed_points = []
  let colors = [];
//   let raw_points_clone = raw_points.map(point => point);
  processPoints(raw_points, colors, processed_points, 0);
  let final_points = [...processed_points.reverse()];

  async function initChart() {
    const ApexCharts = (await import('apexcharts')).default;
    //   console.log(raw_points);
    //   console.log(final_points);
    chart = new ApexCharts(chartEl, {
      chart: { type: 'line', height: 350, toolbar: { show: false } },
      stroke: { curve: 'smooth', width: 2 },
      colors,
      markers: { size: 5 },
      tooltip: { shared: true },
      xaxis: { type: isTimeseries ? 'datetime' : 'category' },
      yaxis: { labels: { formatter: v => v.toFixed(2) } },
      annotations: {
        yaxis: [{
          y: 0,
          borderColor: '#999',
          strokeDashArray: 4,
          label: { text: '0', style: { color: '#fff', background: '#999' } }
        }]
      },
      legend: {
        show: false
      },
      series: final_points.map(points => { return { name: labelName, data: points } })
    });
    chart.render();
  }

  onMount(() => {
    initChart();
    return () => chart?.destroy();
  });

  $: if (chart) {
    chart.updateSeries(
      final_points.map(points => { return { name: labelName, data: points } })
    );
  }

  onDestroy(() => {
    chart?.destroy();
  });
</script>

<div bind:this={chartEl} class="w-full h-[350px]"></div>
