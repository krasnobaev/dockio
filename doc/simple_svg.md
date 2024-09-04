Simple diagram with rectangle and property `master="blaster"`

init.drawio.svg

```svg
<svg host="65bd71144e" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="11px" height="11px" viewBox="-0.5 -0.5 11 11" content="&lt;mxfile&gt;&lt;diagram id=&quot;CqIcI3LFbY63W3ZKgRpy&quot; name=&quot;Page-1&quot;&gt;dZLBcoQgDIafhjvK2um5drt76clDzyip0EFxkK3apy8IqIztxUm+/JD4B0TKbr5pOvB3xUCiHLMZkVeU58VTYb8OLB5csosHrRbMo2wHlfiBAHGgD8FgTIRGKWnEkMJG9T00JmFUazWlsk8l064DbeEEqobKM/0QzHBPnwu88zuIlsfOGQ6VjkZxACOnTE0HRK6IlFop46NuLkE676Iv/tzbP9VtMA29+eOAqr+cHzmWtLYrWQVxstGA9qSWPlkL8ep8m27rOpolOqLVo2fgpBiRl4kLA9VAG1ed7BOwjJvONcxseB4yzP0N2sB8QKHfDVQHRi9WMqcvYUnTad9GRPy4iMBo2H+73btbZIPwgzENpsV0X84qPbxwcv0F&lt;/diagram&gt;&lt;/mxfile&gt;">
    <defs/>
    <g>
        <rect x="0" y="0" width="10" height="10" fill="rgb(255, 255, 255)" stroke="rgb(0, 0, 0)" pointer-events="all"/>
    </g>
</svg>
```

init.drawio

```xml
<mxfile><diagram id="CqIcI3LFbY63W3ZKgRpy" name="Page-1">dZLBcoQgDIafhjvK2um5drt76clDzyip0EFxkK3apy8IqIztxUm+/JD4B0TKbr5pOvB3xUCiHLMZkVeU58VTYb8OLB5csosHrRbMo2wHlfiBAHGgD8FgTIRGKWnEkMJG9T00JmFUazWlsk8l064DbeEEqobKM/0QzHBPnwu88zuIlsfOGQ6VjkZxACOnTE0HRK6IlFop46NuLkE676Iv/tzbP9VtMA29+eOAqr+cHzmWtLYrWQVxstGA9qSWPlkL8ep8m27rOpolOqLVo2fgpBiRl4kLA9VAG1ed7BOwjJvONcxseB4yzP0N2sB8QKHfDVQHRi9WMqcvYUnTad9GRPy4iMBo2H+73btbZIPwgzENpsV0X84qPbxwcv0F</diagram></mxfile>
```

add `compressed=false`

```xml
<mxfile host="65bd71144e">
    <diagram id="CqIcI3LFbY63W3ZKgRpy" name="Page-1">
        <mxGraphModel dx="565" dy="414" grid="1" gridSize="10" guides="1" tooltips="1" connect="1" arrows="1" fold="1" page="1" pageScale="1" pageWidth="850" pageHeight="1100" math="0" shadow="0">
            <root>
                <mxCell id="0"/>
                <mxCell id="1" parent="0"/>
                <object label="" master="blaster" id="2">
                    <mxCell style="rounded=0;whiteSpace=wrap;html=1;" parent="1" vertex="1">
                        <mxGeometry x="10" y="10" width="10" height="10" as="geometry"/>
                    </mxCell>
                </object>
            </root>
        </mxGraphModel>
    </diagram>
</mxfile>
```

nodejs

see `Graph.decompress = function(data, inflate, checked)`

```js
str = 'dZLBcoQgDIafhjvK2um5drt76clDzyip0EFxkK3apy8IqIztxUm+/JD4B0TKbr5pOvB3xUCiHLMZkVeU58VTYb8OLB5csosHrRbMo2wHlfiBAHGgD8FgTIRGKWnEkMJG9T00JmFUazWlsk8l064DbeEEqobKM/0QzHBPnwu88zuIlsfOGQ6VjkZxACOnTE0HRK6IlFop46NuLkE676Iv/tzbP9VtMA29+eOAqr+cHzmWtLYrWQVxstGA9qSWPlkL8ep8m27rOpolOqLVo2fgpBiRl4kLA9VAG1ed7BOwjJvONcxseB4yzP0N2sB8QKHfDVQHRi9WMqcvYUnTad9GRPy4iMBo2H+73btbZIPwgzENpsV0X84qPbxwcv0F';

data = atob(str);
tmp = Uint8Array.from(data, c => c.charCodeAt(0));
pako = require('pako');
text = pako.inflateRaw(tmp, {to: 'string'});
decodeURIComponent(text);
```

result

```xml
<mxGraphModel dx="565" dy="414" grid="1" gridSize="10" guides="1" tooltips="1" connect="1" arrows="1" fold="1" page="1" pageScale="1" pageWidth="850" pageHeight="1100" math="0" shadow="0"><root><mxCell id="0"/><mxCell id="1" parent="0"/><object label="" master="blaster" id="2"><mxCell style="rounded=0;whiteSpace=wrap;html=1;" parent="1" vertex="1"><mxGeometry x="10" y="10" width="10" height="10" as="geometry"/></mxCell></object></root></mxGraphModel>
```

```xml
<mxGraphModel dx="565" dy="414" grid="1" gridSize="10" guides="1" tooltips="1" connect="1" arrows="1" fold="1" page="1" pageScale="1" pageWidth="850" pageHeight="1100" math="0" shadow="0">
  <root>
    <mxCell id="0"/>
    <mxCell id="1" parent="0"/>
    <object label="" master="blaster" id="2">
      <mxCell style="rounded=0;whiteSpace=wrap;html=1;" parent="1" vertex="1">
        <mxGeometry x="10" y="10" width="10" height="10" as="geometry"/>
      </mxCell>
    </object>
  </root>
</mxGraphModel>
```
