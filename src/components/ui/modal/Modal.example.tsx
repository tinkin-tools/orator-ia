import React, { useState } from "react"
import Modal from "./Modal"

/**
 * Ejemplos de uso del componente Modal
 * Este archivo muestra diferentes formas de usar el modal en tu aplicación
 */

export const ModalExamples = () => {
  const [showConfirmModal, setShowConfirmModal] = useState(false)
  const [showDangerModal, setShowDangerModal] = useState(false)
  const [showCustomModal, setShowCustomModal] = useState(false)

  return (
    <div>
      {/* Ejemplo 1: Modal de confirmación básico */}
      <button onClick={() => setShowConfirmModal(true)}>
        Abrir modal de confirmación
      </button>
      <Modal
        isOpen={showConfirmModal}
        onClose={() => setShowConfirmModal(false)}
        title="¿Quieres finalizar la grabación?"
        description="Al aceptar se enviará a analizar y te devolveremos información al respecto."
        confirmText="Finalizar grabación"
        onConfirm={() => {
          console.log("Confirmado!")
          setShowConfirmModal(false)
        }}
      />

      {/* Ejemplo 2: Modal con variante de peligro */}
      <button onClick={() => setShowDangerModal(true)}>
        Abrir modal de eliminación
      </button>
      <Modal
        isOpen={showDangerModal}
        onClose={() => setShowDangerModal(false)}
        title="¿Estás seguro?"
        description="Esta acción no se puede deshacer. Se eliminará permanentemente."
        confirmText="Eliminar"
        cancelText="No, mantener"
        variant="danger"
        onConfirm={() => {
          console.log("Eliminado!")
          setShowDangerModal(false)
        }}
      />

      {/* Ejemplo 3: Modal personalizado */}
      <button onClick={() => setShowCustomModal(true)}>
        Abrir modal personalizado
      </button>
      <Modal
        isOpen={showCustomModal}
        onClose={() => setShowCustomModal(false)}
        title="¿Guardar cambios?"
        description="Tienes cambios sin guardar. ¿Quieres guardarlos antes de salir?"
        confirmText="Guardar y salir"
        cancelText="Descartar cambios"
        onConfirm={() => {
          console.log("Guardado!")
          setShowCustomModal(false)
        }}
      />
    </div>
  )
}
