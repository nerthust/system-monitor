# Monitor del Sistema
**Trabajo realizado en el marco de la clase espejo entre EAFIT y Uniminuto**

En grupos, deben realizar un programa similar al *task manager* (*administrador de tareas*) o el *system monitor* (*Monitor del sistema*) de Windows:tm: o a herramientas como htop o System Monitor en Linux. 

## Descripción
El sistema operativo es una aplicación (software) que cumple dos funciones básicas:
* Le proporciona a las aplicaciones la ilusión de que es el único programa que se ejecuta en la máquina.
* Gestiona todos los recursos de manera que su uso sea apropiado.

En el presente trabajo, deben construir una aplicación que monitoree el uso de los recursos del sistema. Los principales recursos de un sistema son el procesador, la memoria, los archivos y los diferentes dispositivios. Con el fin de poder getionar apropiadamente los recursos, el administrador del sistema necesita monitorearlos. 

Su grupo ha sido contratado para hacer una aplicación que monitorée el funcionamiento del sistema. La aplicación debe presentar periódicamente la informacion de uso de recursos para cada aplicación activa en el sistema durante el intervalo de monitereo. Inicialmente, debe presentar la información cada segundo, pero este valor debe ser programable. 

Para cada aplicación activa en el sistema, la aplicación debe presentar, al menos, la siguiente información:
1. Número de proceso
2. Número del proceso padre
3. Comando utilizado para ejecutarla
4. Cuál es la prioridad del proceso
5. Qué porcentaje de la CPU está utilizando
6. Qué porcentaje de la memoria está utilizando
7. Cuántos bytes ha leido y escrito de disco
8. Cuántos bytes ha enviado y recibido por la red

Mientras más detallada sea la información e incluyan más recursos, será mejor valorada la aplicación.

La interfaz de usuario puede ser gráfica o basada en texto, pero debe ser clara e intuitiva.

**Bonificación:** Hay una bonifición para las aplicaciones que permitan seleccionar un proceso y modificar su prioridad. En este caso, deben conservar la información necesaria para poder evidenciar en una gráfica el impacto que tiene el cambio de prioridad en el proceso. La gráfica puede ser realizada con excel o con otra herramienta. La aplicación es responsable de preservar toda esa información en un archivo. 
