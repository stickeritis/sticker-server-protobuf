use std::iter::FromIterator;

use conllx::graph::{DepTriple, Node, Sentence};
use conllx::token::{Features, Token};

use crate::Metadata;
use crate::Sentence as ProtoSentence;
use crate::Token as ProtoToken;

pub trait FromSentence {
    fn from_sentence(&self, metadata: &Metadata) -> ProtoSentence;
}

impl FromSentence for Sentence {
    fn from_sentence(&self, metadata: &Metadata) -> ProtoSentence {
	let sentence = ProtoSentence::new();

	let layers = metadata.get_layers();

	for (idx, token) in self.iter().filter_map(Node::token).enumerate() {
	    let idx = idx + 1;

	    let mut proto_token = ProtoToken::new();
	    proto_token.form = token.form().to_owned();

	    if layers.lemma {
		proto_token.form = token.lemma().unwrap_or("_").to_owned();
	    }

	    if layers.cpos {
		proto_token.cpos = token.cpos().unwrap_or("_").to_owned();
	    }

	    if layers.pos {
		proto_token.pos = token.pos().unwrap_or("_").to_owned();
	    }

	    if layers.head {
		let triple = self.dep_graph().head(idx);
	    }


	}

	sentence
    }
}


pub trait ToSentence {
    fn to_sentence(&self, metadata: &Metadata) -> Sentence;
}

impl ToSentence for ProtoSentence {
    fn to_sentence(&self, metadata: &Metadata) -> Sentence {
	let layers = metadata.get_layers();
	
	let tokens_iter = self.get_tokens().into_iter().map(|token_proto| {
	    let mut token = Token::new(token_proto.form.clone());

	    if layers.lemma {
		token.set_lemma(Some(token_proto.lemma.clone()));
	    }

	    if layers.cpos {
		token.set_cpos(Some(token_proto.cpos.clone()));
	    }

	    if layers.pos {
		token.set_pos(Some(token_proto.pos.clone()));
	    }

	    let mut features = Features::new();
	    for feature in layers.get_features() {
		if let Some(value) = token_proto.features.get(feature) {
		    features.insert(feature.clone(), Some(value.clone()));
		}
	    }

	    if !features.is_empty() {
		token.set_features(Some(features));
	    }

	    token
	});

	let mut sentence = Sentence::from_iter(tokens_iter);

	if layers.head {
	    let mut dep_graph = sentence.dep_graph_mut();
	    
	    for (idx, token_proto) in self.get_tokens().iter().enumerate() {
		let idx = idx + 1;

		let relation = if layers.deprel {
		    Some(&token_proto.deprel)
		} else {
		    None
		};

		dep_graph.add_deprel(DepTriple::new(token_proto.head as usize, relation, idx));
	    }

	}

	sentence
    }	
}
