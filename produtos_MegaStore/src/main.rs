extern crate petgraph;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::graph::node_index;
use petgraph::algo::dijkstra;
use std::collections::{HashMap, HashSet};

fn main() {
    // Exemplo de transações de compras (produto é identificado por uma string)
    let transacoes = vec![
        vec!["produto_1", "produto_2", "produto_3"],
        vec!["produto_2", "produto_3", "produto_4"],
        vec!["produto_1", "produto_3"],
        vec!["produto_1", "produto_2"],
    ];

    // Construção do grafo
    let mut grafo = DiGraph::new();
    let mut mapa_produto_para_no: HashMap<String, NodeIndex> = HashMap::new();
    let mut co_ocorrencias: HashMap<(String, String), u32> = HashMap::new();

    // Adicionar nós para cada produto
    for transacao in &transacoes {
        for &produto in transacao {
            if !mapa_produto_para_no.contains_key(produto) {
                let no = grafo.add_node(produto.to_string());
                mapa_produto_para_no.insert(produto.to_string(), no);
            }
        }
    }

    // Adicionar arestas (co-ocorrências) entre produtos
    for transacao in transacoes {
        for i in 0..transacao.len() {
            for j in i + 1..transacao.len() {
                let produto_a = transacao[i];
                let produto_b = transacao[j];

                let no_a = *mapa_produto_para_no.get(produto_a).unwrap();
                let no_b = *mapa_produto_para_no.get(produto_b).unwrap();

                // Aumentar a co-ocorrência (peso da aresta)
                let chave = if produto_a < produto_b {
                    (produto_a.to_string(), produto_b.to_string())
                } else {
                    (produto_b.to_string(), produto_a.to_string())
                };

                let contador = co_ocorrencias.entry(chave).or_insert(0);
                *contador += 1;

                grafo.add_edge(no_a, no_b, *contador);
                grafo.add_edge(no_b, no_a, *contador);
            }
        }
    }

    // Imprimir as arestas do grafo (para depuração)
    for aresta in grafo.edge_indices() {
        let (n1, n2) = grafo.edge_endpoints(aresta).unwrap();
        let peso = grafo[aresta];
        println!(
            "Aresta entre {} e {} com peso {}",
            grafo[n1], grafo[n2], peso
        );
    }

    // Função de recomendação
    fn recomendar_produtos(
        produto: &str,
        grafo: &DiGraph<String, u32>,
        mapa_produto_para_no: &HashMap<String, NodeIndex>,
        num_recomendacoes: usize,
    ) -> Vec<String> {
        if let Some(&no_produto) = mapa_produto_para_no.get(produto) {
            let mut vizinhos: Vec<(NodeIndex, u32)> = grafo
                .neighbors(no_produto)
                .map(|no_vizinho| {
                    let peso = grafo[grafo.find_edge(no_produto, no_vizinho).unwrap()];
                    (no_vizinho, peso)
                })
                .collect();

            // Ordenar os vizinhos por peso (co-ocorrência)
            vizinhos.sort_by(|a, b| b.1.cmp(&a.1));

            // Retornar os produtos mais recomendados
            vizinhos.iter().take(num_recomendacoes).map(|(no_vizinho, _)| grafo[*no_vizinho].clone()).collect()
        } else {
            vec![]
        }
    }

    // Exemplo de recomendação para 'produto_1'
    let produtos_recomendados = recomendar_produtos("produto_1", &grafo, &mapa_produto_para_no, 3);
    println!("\nProdutos recomendados para 'produto_1': {:?}", produtos_recomendados);
}


  // Mostrar recomendação detalhada
  if !produtos_recomendados.is_empty() {
    println!("\nRecomendações para o produto 'produto_1':");
    for (i, produto) in produtos_recomendados.iter().enumerate() {
        println!("{}. {}", i + 1, produto);
    }
} else {
    println!("\nNenhuma recomendação encontrada para o produto 'produto_1'.");
}
