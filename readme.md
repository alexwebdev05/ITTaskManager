# Gestor de Tareas IT - Proyecto de aprendizaje Rust

Proyecto CLI para practicar conceptos fundamentales de Rust: enums, structs, impl blocks, ownership, y persistencia de datos.

## Checklist de implementación

### Fase 1: Estructuras básicas
- [*] Crear los enums `TaskPriority` y `TaskStatus`
- [*] Crear la struct `Task` con sus campos básicos

### Fase 2: Métodos de Task
- [*] Constructor `new()`
- [*] `display()` para mostrar info
- [ ] `start()`, `complete()`, `block()` para cambiar estados
- [ ] `is_completed()` para verificar estado

### Fase 3: TaskManager
- [ ] Crear la struct `TaskManager`
- [ ] Constructor `new()`
- [ ] `add_task()` para crear tareas
- [ ] `list_tasks()` para mostrar todas
- [ ] `find_task_mut()` para buscar por ID
- [ ] `list_by_priority()` para filtrar

### Fase 4: Interfaz CLI
- [ ] Crear nueva tarea
- [ ] Listar tareas
- [ ] Cambiar estado de tarea
- [ ] Filtrar por prioridad
- [ ] Salir del programa

### Fase 5: Persistencia
- [ ] `save_to_file()` para guardar en JSON local
- [ ] `load_from_file()` para cargar al iniciar
- [ ] Usar `serde` y `serde_json` para serialización
- [ ] Manejar errores con `Result<T, E>` en operaciones de archivo

## Dependencias necesarias
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Uso
```bash
cargo run
```