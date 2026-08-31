from graphviz import Digraph

graph = Digraph(
    "compiler_pipeline",
    format="png",
)

graph.attr(
    rankdir="TB",
    bgcolor="white",
    pad="0.4",
    nodesep="0.55",
    ranksep="0.7",
    splines="ortho",
    fontname="Arial",
)
graph.attr(
    "node",
    shape="box",
    style="rounded,filled",
    fontname="Arial",
    fontsize="14",
    fontcolor="#172033",
    color="#CBD5E1",
    penwidth="1.5",
    margin="0.18,0.12",
)
graph.attr(
    "edge",
    fontname="Arial",
    fontsize="11",
    fontcolor="#475569",
    color="#475569",
    penwidth="1.4",
    arrowsize="0.7",
)


ICON_SIZE = "36"


def icon_node(g, name, icon, label, fill, border, icon_size=ICON_SIZE):
    label_html = label.replace("\n", "<BR/>")
    html_label = (
        '<<TABLE BORDER="0" CELLBORDER="0" CELLSPACING="2" CELLPADDING="1">'
        f'<TR><TD WIDTH="{icon_size}" HEIGHT="{icon_size}" FIXEDSIZE="TRUE">'
        f'<IMG SRC="icons/{icon}.png" SCALE="TRUE"/></TD></TR>'
        f'<TR><TD><FONT POINT-SIZE="14">{label_html}</FONT></TD></TR>'
        "</TABLE>>"
    )
    g.node(
        name,
        html_label,
        fillcolor=fill,
        color=border,
        shape="box",
        style="rounded,filled",
        width="1.0",
        height="1.0",
    )


icon_node(graph, "source", "source", "Source (.rp)", "#F5EEFF", "#7C3AED")
icon_node(graph, "lexer", "lexer", "Lexer", "#EEF4FF", "#2563EB")
icon_node(graph, "parser", "parser", "Parser", "#EEF4FF", "#2563EB")

graph.edge("source", "lexer", xlabel=" chars ")
graph.edge("lexer", "parser", xlabel=" tokens ")

with graph.subgraph(name="cluster_import") as import_cluster:
    import_cluster.attr(
        label="  Import Resolver  ",
        color="#8B5CF6",
        fontcolor="#5B21B6",
        fontsize="16",
        fontname="Arial",
        style="rounded,dashed",
        penwidth="1.5",
        margin="20",
    )
    import_cluster.attr(
        "node",
        fontname="Arial",
    )

    with import_cluster.subgraph(name="cluster_recursive") as recursive:
        recursive.attr(
            label="for each import: recurse",
            color="#A78BFA",
            fontcolor="#5B21B6",
            fontsize="13",
            fontname="Arial",
            style="rounded",
            penwidth="1.2",
        )
        icon_node(recursive, "sub_lexer", "lexer", "Lexer", "#F1F5FF", "#2563EB")
        icon_node(recursive, "sub_parser", "parser", "Parser", "#F1F5FF", "#2563EB")
        recursive.edge("sub_lexer", "sub_parser", xlabel=" tokens ")

    icon_node(
        import_cluster,
        "merge",
        "import",
        "merge into AST\n(cycle + cache check)",
        "#F6EEFF",
        "#7C3AED",
    )
    import_cluster.edge("sub_parser", "merge", xlabel=" imported AST ")

graph.edge("parser", "merge", xlabel=" AST ")

icon_node(graph, "checker", "checker", "Semantic Checker", "#F0FDF4", "#16A34A")
graph.edge("merge", "checker", xlabel=" merged AST ")

with graph.subgraph(name="cluster_backends") as backends:
    backends.attr(
        label="  Backends  ",
        color="#F59E0B",
        fontcolor="#A16207",
        fontsize="16",
        fontname="Arial",
        style="rounded,dashed",
        penwidth="1.5",
        margin="20",
    )

    icon_node(
        backends, "interpreter", "interpreter", "Interpreter", "#FFF7E6", "#F59E0B"
    )
    icon_node(backends, "compiler", "compiler", "Compiler", "#FFF7E6", "#F59E0B")
    icon_node(backends, "llvm", "llvm", "LLVM 18 tools", "#F8FAFC", "#64748B")
    icon_node(backends, "executable", "executable", "executable", "#F5EEFF", "#7C3AED")

    backends.edge("compiler", "llvm", xlabel=" LLVM IR ")
    backends.edge("llvm", "executable")

    with backends.subgraph() as same_rank:
        same_rank.attr(rank="same")
        same_rank.node("interpreter")
        same_rank.node("compiler")

graph.edge("checker", "interpreter", xlabel=" checked AST ")
graph.edge("checker", "compiler", xlabel=" checked AST ")

graph.render(
    "compiler_pipeline",
    cleanup=True,
)
