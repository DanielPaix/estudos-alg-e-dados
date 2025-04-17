use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;
use petgraph::visit::EdgeRef;


fn main() {
    let mut graph = DiGraph::<&str, f32>::new();

    // Adicionando produtos como nós
    let produto_a = graph.add_node("Produto A");
    let produto_b = graph.add_node("Produto B");
    let produto_c = graph.add_node("Produto C");
    let produto_d = graph.add_node("Produto D");

    // Adicionando arestas com pesos indicando força da relação
    graph.add_edge(produto_a, produto_b, 0.9);
    graph.add_edge(produto_a, produto_c, 0.7);
    graph.add_edge(produto_b, produto_d, 0.8);
    graph.add_edge(produto_c, produto_d, 0.6);

    // Produto de entrada para recomendação
    let input_produto = produto_a;
    
    // Obter recomendações com base nas conexões
    let recomendacoes = recomendar_produtos(&graph, input_produto);
    
    println!("Recomendações para {:?}:", graph[input_produto]);
    for (produto, peso) in recomendacoes {
        println!("{} (relevância: {:.2})", produto, peso);
    }
}

fn recomendar_produtos<'a>(graph: &'a DiGraph<&'a str, f32>, produto: NodeIndex) -> Vec<(&'a str, f32)> {

    let mut recomendacoes = HashMap::new();
    
    for edge in graph.edges(produto) {
        let target = edge.target();
        let peso = *edge.weight();
        recomendacoes.insert(graph[target], peso);
    }
    
    let mut recomendacoes: Vec<_> = recomendacoes.into_iter().collect();
    recomendacoes.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    recomendacoes
}

