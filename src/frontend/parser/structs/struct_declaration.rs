use crate::{
    common::{errors::IError, span::Span},
    frontend::{
        ast::{Node, StructDeclaration, StructMember},
        lexer::lexer::ILexer,
        parser::{
            core::{try_consume, try_consume_token},
            Parser,
        },
        tokens::TokenCategory,
    },
};

impl<L: ILexer> Parser<L> {
    pub(in crate::frontend::parser) fn parse_struct_declaration(&mut self) -> Result<Option<Node<StructDeclaration>>, Box<dyn IError>> {
        // struct_declaration = "struct", identifier, "{", [ struct_members ], "}", ";";
        let struct_token = try_consume_token!(self, TokenCategory::Struct);

        let identifier = self
            .parse_identifier()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't create identifier while parsing struct declaration.")))?;

        let _ = self.consume_must_be(TokenCategory::BraceOpen)?;
        let members = self.parse_members()?;
        let _ = self.consume_must_be(TokenCategory::BraceClose)?;
        let semicolon_token = self.consume_must_be(TokenCategory::Semicolon)?;

        Ok(Some(Node {
            value: StructDeclaration { identifier, members },
            span: Span::new(struct_token.span.start(), semicolon_token.span.end()),
        }))
    }

    fn parse_members(&mut self) -> Result<Vec<Node<StructMember>>, Box<dyn IError>> {
        // struct_members = struct_member, { ",", struct_member };
        let mut members = Vec::new();

        let Some(first_member) = self.parse_member()? else {
            return Ok(members);
        };
        members.push(first_member);

        while self.current_token().category == TokenCategory::Comma {
            let _ = self.consume_must_be(TokenCategory::Comma)?;

            let member = self
                .parse_member()?
                .ok_or_else(|| self.create_parser_error(String::from("Expected struct member after comma.")))?;

            members.push(member);
        }

        Ok(members)
    }
    fn parse_member(&mut self) -> Result<Option<Node<StructMember>>, Box<dyn IError>> {
        // struct_member = type, identifier;
        let t = try_consume!(self, parse_type);
        let identifier = self
            .parse_identifier()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't create identifier while parsing struct member.")))?;

        Ok(Some(Node {
            value: StructMember {
                identifier: identifier.clone(),
                member_type: t.clone(),
            },
            span: Span::new(t.span.start(), identifier.span.end()),
        }))
    }
}
