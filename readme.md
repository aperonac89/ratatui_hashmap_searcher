# Ratatui Terminal Application

## Descripción

Aplicación de terminal interactiva desarrollada con Rust y la librería Ratatui. Esta aplicación presenta una interfaz de usuario en modo texto con dos columnas: una para la lista de categorías (Keys) y otra para los elementos asociados a la categoría seleccionada (Values). Incluye funcionalidades de búsqueda, navegación y cambio de foco entre las columnas.

## Características

- **Interfaz de usuario en terminal**: Diseño moderno con bordes, colores y estilos
- **Navegación interactiva**: Movimiento con flechas arriba/abajo y tabulador para cambiar entre columnas
- **Filtrado en tiempo real**: Búsqueda dinámica en ambas columnas
- **Sistema de foco**: Indicador visual del elemento actualmente seleccionado
- **Fecha y hora actual**: Muestra la fecha y hora en el encabezado
- **Controles de teclado intuitivos**: 
  - Tab: Cambiar entre columnas
  - Flechas: Navegar elementos
  - Backspace: Eliminar caracteres del filtro
  - ESC: Limpiar filtro o salir
  - Ctrl+C: Salir de la aplicación

## Captura de Pantalla

![Terminal Application Screenshot](https://placehold.co/800x600/2D3748/FFFFFF?text=Ratatui+Terminal+App)

*Vista general de la aplicación en ejecución*

## Estructura de Datos

La aplicación muestra datos organizados en categorías:
- **Categorías**: Frutas, Vegetales, Lácteos
- **Elementos**: 
  - Frutas: Apple, Orange, Banana
  - Vegetales: Letuce, Ruccola, Potatoe
  - Lácteos: Milk, Yogurt, Cheese

## Controles de Teclado

| Acción | Tecla |
|--------|-------|
| Cambiar entre columnas | Tab |
| Mover hacia arriba | Flecha Arriba |
| Mover hacia abajo | Flecha Abajo |
| Eliminar último carácter del filtro | Backspace |
| Limpiar filtro actual | ESC |
| Salir de la aplicación | Ctrl+C |

## Requisitos

- Rust 1.70+
- Cargo (gestor de paquetes de Rust)
- Terminal compatible con ANSI

## Instalación y Ejecución

### Clonar el repositorio
```bash
git clone <repositorio>
cd ratatui-app
```

### Compilar y ejecutar
```bash
cargo run
```

## Funcionalidades

1. **Navegación por categorías**: Selecciona una categoría en la columna izquierda para ver sus elementos
2. **Filtrado de datos**: Escribe en cualquiera de las columnas para filtrar los resultados
3. **Cambio de foco**: Usa Tab para mover el cursor entre las dos columnas
4. **Indicadores visuales**: Los elementos seleccionados se resaltan con colores distintos

## Estructura del Proyecto

```
.
├── src/
│   └── main.rs          # Punto de entrada principal
├── Cargo.toml           # Dependencias y configuración del proyecto
└── README.md            # Este archivo
```

## Desarrollo

### Para contribuir:
1. Fork el repositorio
2. Crea una rama para tu feature (`git checkout -b feature/nueva-feature`)
3. Haz commit de tus cambios (`git commit -am 'Agrega nueva feature'`)
4. Push a la rama (`git push origin feature/nueva-feature`)
5. Crea un Pull Request

## Licencia

[MIT License](LICENSE)

## Notas

Esta aplicación demuestra el uso avanzado de Ratatui para crear interfaces de usuario interactivas en terminal, incluyendo:
- Manejo de estado
- Renderizado de widgets complejos
- Eventos de teclado
- Estilos y colores
- Layouts responsivos