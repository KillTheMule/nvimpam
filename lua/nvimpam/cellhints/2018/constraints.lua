return {
OTMCO = {
  { "IDOTM", "OTMCO entity identification number.", "", "integer (I8)", "1" },
  { "IDNODd", "Identification number of the dependent node.", "", "integer (I8)", "1" },
  { "DOFCOD", "DOF constraints for dependent node",
    [[
Flag for constraining each DOF of the dependent node. Only 111000 and 111111 are allowed.

6-digit binary number where each digit corresponds to the state of a particular degree of freedom. The meaning of each digit is as follows:

Column Degree Of Freedom
27     X-axis, translation
28     Y-axis, translation
29     Z-axis, translation
30     X-axis, rotation
31     Y-axis, rotation
32     Z-axis, rotation

Allowed digit values:

  * 0: free
  * 1: constrained
      ]], "binary (I6)", "" },
      { "IMETH", "Flag for transfer of moments",
        [[
Flag to define transfer of moments from the dependent nodes to the independent nodes.

Allowed values:

* 0: Moments on the dependent nodes are transferred by forces onto independent nodes.
* 1: Moments on the dependent nodes are transferred by moments onto independent nodes.
        ]], "integer (I8)", "" },

  { "RADIUS", "Limit initial independent node selection",
    [[
The initial selection of independent nodes is limited to the nodes that lie within the specified radius around the dependent node.

Default value:

  * 0.0: the initial selection is not limited.
    ]], "floating (E8)", "[length]" },
  { "IELIM", "OTMCO elimination criterion.",
    [[
Allowed values:

* 0: OTMCO is removed as soon as one independent node becomes free
* 1: OTMCO is removed when all independent nodes become free.

Default value:

              * 0
    ]], "integer (I8)", "1" },
  { "ITYP", "Type of OTMCO.",
    [[
Allowed values:

  * 0 (default): general OTMCO;
  * 1: OTMCO type barycenter.
    ]], "integer (I8)", "1" },
  { "ALPHA", "Constant thermal expansion coefficient.", "", "floating (E8)", "[temperature^-1]" },
  { "TITLE", "Descriptive title.", "", "string (A76)", "" },
  { "WEIGHT", "Sub-keyword for applying weighting factor", "to the node selection block that follows", "keyword (A6)", ""},
  { "WTFAC", "Weighting factor.", "Default value: 1.0. Free format.", "floating(E)", "1" },
}
}
