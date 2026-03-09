use anchor_lang::prelude::*;

declare_id!("EupWg9myqqUxyicW1odfQiSwwri3VnjyFTn9brpbxZnU");

#[program]
pub mod mesh_node {
    use super::*;

    pub fn crear_red(ctx: Context<CrearRed>, nombre: String, descripcion: String) -> Result<()> {
        validar_tamano(&nombre, 40, Errores::NombreMuyLargo)?;
        validar_tamano(&descripcion, 120, Errores::DescripcionMuyLarga)?;

        let red = &mut ctx.accounts.red;
        red.authority = ctx.accounts.authority.key();
        red.nombre = nombre;
        red.descripcion = descripcion;
        red.total_nodos = 0;
        red.activa = true;

        msg!("Red creada correctamente");
        Ok(())
    }

    // =========================
    // UPDATE - RED
    // =========================
    pub fn actualizar_red(
        ctx: Context<ActualizarRed>,
        nombre: String,
        descripcion: String,
        activa: bool,
    ) -> Result<()> {
        validar_tamano(&nombre, 40, Errores::NombreMuyLargo)?;
        validar_tamano(&descripcion, 120, Errores::DescripcionMuyLarga)?;

        let red = &mut ctx.accounts.red;
        red.nombre = nombre;
        red.descripcion = descripcion;
        red.activa = activa;

        msg!("Red actualizada correctamente");
        Ok(())
    }

    // =========================
    // READ - RED
    // =========================
    pub fn ver_red(ctx: Context<VerRed>) -> Result<()> {
        let red = &ctx.accounts.red;

        msg!("===== RED =====");
        msg!("Authority: {}", red.authority);
        msg!("Nombre: {}", red.nombre);
        msg!("Descripcion: {}", red.descripcion);
        msg!("Total nodos: {}", red.total_nodos);
        msg!("Activa: {}", red.activa);

        Ok(())
    }

    // =========================
    // DELETE - RED
    // Solo se puede eliminar si no tiene nodos registrados
    // =========================
    pub fn eliminar_red(_ctx: Context<EliminarRed>) -> Result<()> {
        msg!("Red eliminada correctamente");
        Ok(())
    }

    // =========================
    // CREATE - NODO
    // =========================
    pub fn registrar_nodo(
        ctx: Context<RegistrarNodo>,
        alias: String,
        tipo: String,
        ubicacion: String,
    ) -> Result<()> {
        validar_tamano(&alias, 32, Errores::AliasMuyLargo)?;
        validar_tamano(&tipo, 20, Errores::TipoMuyLargo)?;
        validar_tamano(&ubicacion, 60, Errores::UbicacionMuyLarga)?;

        require!(ctx.accounts.red.activa, Errores::RedInactiva);

        let nodo = &mut ctx.accounts.nodo;
        nodo.red = ctx.accounts.red.key();
        nodo.owner = ctx.accounts.owner.key();
        nodo.alias = alias;
        nodo.tipo = tipo;
        nodo.ubicacion = ubicacion;
        nodo.activo = true;
        nodo.fecha_registro = Clock::get()?.unix_timestamp;

        ctx.accounts.red.total_nodos = ctx
            .accounts
            .red
            .total_nodos
            .checked_add(1)
            .ok_or(Errores::Overflow)?;

        msg!("Nodo registrado correctamente");
        Ok(())
    }

    // =========================
    // UPDATE - NODO
    // =========================
    pub fn actualizar_nodo(
        ctx: Context<ActualizarNodo>,
        alias: String,
        tipo: String,
        ubicacion: String,
    ) -> Result<()> {
        validar_tamano(&alias, 32, Errores::AliasMuyLargo)?;
        validar_tamano(&tipo, 20, Errores::TipoMuyLargo)?;
        validar_tamano(&ubicacion, 60, Errores::UbicacionMuyLarga)?;

        let nodo = &mut ctx.accounts.nodo;
        nodo.alias = alias;
        nodo.tipo = tipo;
        nodo.ubicacion = ubicacion;

        msg!("Nodo actualizado correctamente");
        Ok(())
    }

    // =========================
    // UPDATE - ESTADO NODO
    // =========================
    pub fn alternar_estado_nodo(ctx: Context<AlternarEstadoNodo>) -> Result<()> {
        let nodo = &mut ctx.accounts.nodo;
        nodo.activo = !nodo.activo;

        msg!("Nuevo estado del nodo: {}", nodo.activo);
        Ok(())
    }

    // =========================
    // READ - NODO
    // =========================
    pub fn ver_nodo(ctx: Context<VerNodo>) -> Result<()> {
        let nodo = &ctx.accounts.nodo;

        msg!("===== NODO =====");
        msg!("Red: {}", nodo.red);
        msg!("Owner: {}", nodo.owner);
        msg!("Alias: {}", nodo.alias);
        msg!("Tipo: {}", nodo.tipo);
        msg!("Ubicacion: {}", nodo.ubicacion);
        msg!("Activo: {}", nodo.activo);
        msg!("Fecha registro: {}", nodo.fecha_registro);

        Ok(())
    }

    // =========================
    // DELETE - NODO
    // Cierra la cuenta y devuelve la renta al owner
    // =========================
    pub fn eliminar_nodo(ctx: Context<EliminarNodo>) -> Result<()> {
        ctx.accounts.red.total_nodos = ctx
            .accounts
            .red
            .total_nodos
            .checked_sub(1)
            .ok_or(Errores::Underflow)?;

        msg!("Nodo eliminado correctamente");
        Ok(())
    }
}

// =====================================================
// CUENTAS
// =====================================================

#[account]
#[derive(InitSpace)]
pub struct Red {
    pub authority: Pubkey, // 32 bytes

    #[max_len(40)]
    pub nombre: String,

    #[max_len(120)]
    pub descripcion: String,

    pub total_nodos: u32, // 4 bytes
    pub activa: bool,     // 1 byte
}

#[account]
#[derive(InitSpace)]
pub struct Nodo {
    pub red: Pubkey,   // 32 bytes
    pub owner: Pubkey, // 32 bytes

    #[max_len(32)]
    pub alias: String,

    #[max_len(20)]
    pub tipo: String,

    #[max_len(60)]
    pub ubicacion: String,

    pub activo: bool,        // 1 byte
    pub fecha_registro: i64, // 8 bytes
}

// =====================================================
// CONTEXTOS - RED
// =====================================================

#[derive(Accounts)]
pub struct CrearRed<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + Red::INIT_SPACE,
        seeds = [b"red", authority.key().as_ref()],
        bump
    )]
    pub red: Account<'info, Red>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActualizarRed<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"red", authority.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub red: Account<'info, Red>,
}

#[derive(Accounts)]
pub struct VerRed<'info> {
    pub red: Account<'info, Red>,
}

#[derive(Accounts)]
pub struct EliminarRed<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        close = authority,
        seeds = [b"red", authority.key().as_ref()],
        bump,
        has_one = authority,
        constraint = red.total_nodos == 0 @ Errores::RedConNodos
    )]
    pub red: Account<'info, Red>,
}

// =====================================================
// CONTEXTOS - NODO
// =====================================================

#[derive(Accounts)]
pub struct RegistrarNodo<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub red: Account<'info, Red>,

    #[account(
        init,
        payer = owner,
        space = 8 + Nodo::INIT_SPACE,
        seeds = [b"nodo", red.key().as_ref(), owner.key().as_ref()],
        bump
    )]
    pub nodo: Account<'info, Nodo>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActualizarNodo<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    pub red: Account<'info, Red>,

    #[account(
        mut,
        seeds = [b"nodo", red.key().as_ref(), owner.key().as_ref()],
        bump,
        has_one = owner,
        has_one = red
    )]
    pub nodo: Account<'info, Nodo>,
}

#[derive(Accounts)]
pub struct AlternarEstadoNodo<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    pub red: Account<'info, Red>,

    #[account(
        mut,
        seeds = [b"nodo", red.key().as_ref(), owner.key().as_ref()],
        bump,
        has_one = owner,
        has_one = red
    )]
    pub nodo: Account<'info, Nodo>,
}

#[derive(Accounts)]
pub struct VerNodo<'info> {
    pub red: Account<'info, Red>,

    #[account(
        has_one = red
    )]
    pub nodo: Account<'info, Nodo>,
}

#[derive(Accounts)]
pub struct EliminarNodo<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub red: Account<'info, Red>,

    #[account(
        mut,
        close = owner,
        seeds = [b"nodo", red.key().as_ref(), owner.key().as_ref()],
        bump,
        has_one = owner,
        has_one = red
    )]
    pub nodo: Account<'info, Nodo>,
}

// =====================================================
// ERRORES
// =====================================================

#[error_code]
pub enum Errores {
    #[msg("El nombre excede el limite permitido")]
    NombreMuyLargo,

    #[msg("La descripcion excede el limite permitido")]
    DescripcionMuyLarga,

    #[msg("El alias excede el limite permitido")]
    AliasMuyLargo,

    #[msg("El tipo excede el limite permitido")]
    TipoMuyLargo,

    #[msg("La ubicacion excede el limite permitido")]
    UbicacionMuyLarga,

    #[msg("La red esta inactiva")]
    RedInactiva,

    #[msg("No puedes eliminar la red porque aun tiene nodos registrados")]
    RedConNodos,

    #[msg("Overflow aritmetico")]
    Overflow,

    #[msg("Underflow aritmetico")]
    Underflow,
}

// =====================================================
// FUNCIONES AUXILIARES
// =====================================================

fn validar_tamano(texto: &str, max: usize, error: Errores) -> Result<()> {
    if texto.as_bytes().len() > max {
        return Err(error.into());
    }
    Ok(())
}

