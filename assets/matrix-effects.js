/**
 * Matrix Console Effects Library
 * Provides typewriter animations, CRT effects, and Matrix-style visual enhancements
 */

class MatrixEffects {
  constructor() {
    this.isInitialized = false;
    this.typewriterQueue = [];
    this.isTyping = false;
  }

  /**
   * Initialize Matrix effects
   */
  init() {
    if (this.isInitialized) return;
    
    this.createMatrixRain();
    this.initCRTEffects();
    this.initGlitchEffects();
    this.isInitialized = true;
  }

  /**
   * Create Matrix digital rain effect
   */
  createMatrixRain() {
    const canvas = document.createElement('canvas');
    canvas.id = 'matrix-rain';
    canvas.style.position = 'fixed';
    canvas.style.top = '0';
    canvas.style.left = '0';
    canvas.style.width = '100%';
    canvas.style.height = '100%';
    canvas.style.pointerEvents = 'none';
    canvas.style.zIndex = '-1';
    canvas.style.opacity = '0.1';
    
    document.body.appendChild(canvas);
    
    const ctx = canvas.getContext('2d');
    
    // Set canvas size
    const resizeCanvas = () => {
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;
    };
    
    resizeCanvas();
    window.addEventListener('resize', resizeCanvas);
    
    // Matrix characters
    const matrixChars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789@#$%^&*()_+-=[]{}|;:,.<>?';
    const fontSize = 14;
    const columns = canvas.width / fontSize;
    const drops = Array(Math.floor(columns)).fill(1);
    
    // Animation function
    const draw = () => {
      ctx.fillStyle = 'rgba(0, 0, 0, 0.04)';
      ctx.fillRect(0, 0, canvas.width, canvas.height);
      
      ctx.fillStyle = '#00FF00';
      ctx.font = `${fontSize}px Courier Prime, Courier New, monospace`;
      
      for (let i = 0; i < drops.length; i++) {
        const text = matrixChars[Math.floor(Math.random() * matrixChars.length)];
        ctx.fillText(text, i * fontSize, drops[i] * fontSize);
        
        if (drops[i] * fontSize > canvas.height && Math.random() > 0.975) {
          drops[i] = 0;
        }
        drops[i]++;
      }
    };
    
    // Start animation
    setInterval(draw, 35);
  }

  /**
   * Initialize CRT screen effects
   */
  initCRTEffects() {
    // Add CRT scanlines
    const style = document.createElement('style');
    style.textContent = `
      .crt-scanlines::after {
        content: '';
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: repeating-linear-gradient(
          0deg,
          transparent,
          transparent 2px,
          rgba(0, 255, 0, 0.03) 2px,
          rgba(0, 255, 0, 0.03) 4px
        );
        pointer-events: none;
        z-index: 1000;
      }
      
      .crt-flicker {
        animation: crt-flicker 0.15s infinite linear;
      }
      
      @keyframes crt-flicker {
        0%, 19.999%, 22%, 62.999%, 64%, 64.999%, 70%, 100% {
          opacity: 1;
        }
        20%, 21.999%, 63%, 63.999%, 65%, 69.999% {
          opacity: 0.4;
        }
      }
    `;
    document.head.appendChild(style);
  }

  /**
   * Initialize glitch effects
   */
  initGlitchEffects() {
    const style = document.createElement('style');
    style.textContent = `
      .matrix-glitch {
        position: relative;
        display: inline-block;
      }
      
      .matrix-glitch::before,
      .matrix-glitch::after {
        content: attr(data-text);
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
      }
      
      .matrix-glitch::before {
        animation: glitch-1 0.5s infinite;
        color: #ff0000;
        z-index: -1;
      }
      
      .matrix-glitch::after {
        animation: glitch-2 0.5s infinite;
        color: #0000ff;
        z-index: -2;
      }
      
      @keyframes glitch-1 {
        0%, 14%, 15%, 49%, 50%, 99%, 100% {
          transform: translate(0);
        }
        15%, 49% {
          transform: translate(-2px, 2px);
        }
      }
      
      @keyframes glitch-2 {
        0%, 20%, 21%, 62%, 63%, 99%, 100% {
          transform: translate(0);
        }
        21%, 62% {
          transform: translate(2px, -2px);
        }
      }
    `;
    document.head.appendChild(style);
  }

  /**
   * Typewriter effect for text elements
   * @param {HTMLElement} element - Target element
   * @param {string} text - Text to type
   * @param {number} speed - Typing speed in milliseconds
   * @param {Function} callback - Callback function when complete
   */
  typewriter(element, text, speed = 50, callback = null) {
    this.typewriterQueue.push({ element, text, speed, callback });
    this.processTypewriterQueue();
  }

  /**
   * Process typewriter queue
   */
  async processTypewriterQueue() {
    if (this.isTyping || this.typewriterQueue.length === 0) return;
    
    this.isTyping = true;
    const { element, text, speed, callback } = this.typewriterQueue.shift();
    
    element.textContent = '';
    element.style.borderRight = '2px solid #00FF00';
    
    for (let i = 0; i <= text.length; i++) {
      element.textContent = text.slice(0, i);
      await this.delay(speed);
    }
    
    // Blink cursor for a moment
    await this.delay(500);
    element.style.borderRight = 'none';
    
    if (callback) callback();
    
    this.isTyping = false;
    this.processTypewriterQueue(); // Process next in queue
  }

  /**
   * Animate text appearance with Matrix-style effects
   * @param {HTMLElement} element - Target element
   * @param {string} finalText - Final text to display
   * @param {number} duration - Animation duration in milliseconds
   */
  matrixTextReveal(element, finalText, duration = 1000) {
    const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789@#$%^&*()';
    const iterations = Math.floor(duration / 50);
    let iteration = 0;
    
    const interval = setInterval(() => {
      element.textContent = finalText
        .split('')
        .map((char, index) => {
          if (index < iteration) {
            return finalText[index];
          }
          return chars[Math.floor(Math.random() * chars.length)];
        })
        .join('');
      
      if (iteration >= finalText.length) {
        clearInterval(interval);
      }
      
      iteration += 1 / 3;
    }, 50);
  }

  /**
   * Add glitch effect to element
   * @param {HTMLElement} element - Target element
   * @param {number} duration - Glitch duration in milliseconds
   */
  glitchEffect(element, duration = 1000) {
    const originalText = element.textContent;
    element.setAttribute('data-text', originalText);
    element.classList.add('matrix-glitch');
    
    setTimeout(() => {
      element.classList.remove('matrix-glitch');
      element.removeAttribute('data-text');
    }, duration);
  }

  /**
   * Create Matrix-style loading animation
   * @param {HTMLElement} container - Container element
   * @param {string} message - Loading message
   */
  createLoadingAnimation(container, message = 'LOADING') {
    const loadingDiv = document.createElement('div');
    loadingDiv.className = 'matrix-loading-container';
    loadingDiv.style.cssText = `
      display: flex;
      align-items: center;
      gap: 1rem;
      color: #00FF00;
      font-family: 'Courier Prime', 'Courier New', monospace;
      text-shadow: 0 0 5px currentColor;
    `;
    
    const spinner = document.createElement('div');
    spinner.className = 'matrix-loading';
    
    const text = document.createElement('span');
    text.textContent = message;
    
    const dots = document.createElement('span');
    dots.style.animation = 'matrix-loading-dots 1.5s infinite';
    
    // Animate dots
    let dotCount = 0;
    const dotInterval = setInterval(() => {
      dots.textContent = '.'.repeat((dotCount % 3) + 1);
      dotCount++;
    }, 500);
    
    loadingDiv.appendChild(spinner);
    loadingDiv.appendChild(text);
    loadingDiv.appendChild(dots);
    container.appendChild(loadingDiv);
    
    return {
      element: loadingDiv,
      destroy: () => {
        clearInterval(dotInterval);
        if (loadingDiv.parentNode) {
          loadingDiv.parentNode.removeChild(loadingDiv);
        }
      }
    };
  }

  /**
   * Create Matrix-style notification
   * @param {string} message - Notification message
   * @param {string} type - Notification type ('success', 'error', 'warning', 'info')
   * @param {number} duration - Display duration in milliseconds
   */
  showNotification(message, type = 'info', duration = 3000) {
    const notification = document.createElement('div');
    notification.className = `matrix-notification matrix-notification-${type}`;
    
    const colors = {
      success: '#00FF00',
      error: '#FF0000',
      warning: '#FFFF00',
      info: '#00CCFF'
    };
    
    notification.style.cssText = `
      position: fixed;
      top: 20px;
      right: 20px;
      background: rgba(0, 0, 0, 0.9);
      border: 1px solid ${colors[type]};
      color: ${colors[type]};
      padding: 1rem 1.5rem;
      border-radius: 0.5rem;
      font-family: 'Courier Prime', 'Courier New', monospace;
      text-shadow: 0 0 5px currentColor;
      box-shadow: 0 0 10px ${colors[type]}40;
      z-index: 10000;
      transform: translateX(100%);
      transition: transform 0.3s ease;
    `;
    
    notification.textContent = message;
    document.body.appendChild(notification);
    
    // Animate in
    setTimeout(() => {
      notification.style.transform = 'translateX(0)';
    }, 10);
    
    // Animate out and remove
    setTimeout(() => {
      notification.style.transform = 'translateX(100%)';
      setTimeout(() => {
        if (notification.parentNode) {
          notification.parentNode.removeChild(notification);
        }
      }, 300);
    }, duration);
  }

  /**
   * Add Matrix hover effect to buttons
   * @param {HTMLElement} button - Button element
   */
  addButtonEffect(button) {
    button.addEventListener('mouseenter', () => {
      button.style.textShadow = '0 0 10px #00FF00';
      button.style.boxShadow = '0 0 20px rgba(0, 255, 0, 0.3)';
    });
    
    button.addEventListener('mouseleave', () => {
      button.style.textShadow = '0 0 5px currentColor';
      button.style.boxShadow = 'none';
    });
    
    button.addEventListener('click', () => {
      button.style.transform = 'scale(0.98)';
      setTimeout(() => {
        button.style.transform = 'scale(1)';
      }, 100);
    });
  }

  /**
   * Create Matrix-style progress bar
   * @param {HTMLElement} container - Container element
   * @param {number} progress - Progress value (0-100)
   */
  createProgressBar(container, progress = 0) {
    const progressBar = document.createElement('div');
    progressBar.className = 'matrix-progress-bar';
    progressBar.style.cssText = `
      width: 100%;
      height: 20px;
      background: rgba(0, 255, 0, 0.1);
      border: 1px solid #008800;
      border-radius: 0.25rem;
      overflow: hidden;
      position: relative;
    `;
    
    const progressFill = document.createElement('div');
    progressFill.className = 'matrix-progress-fill';
    progressFill.style.cssText = `
      height: 100%;
      background: linear-gradient(90deg, #004400, #00FF00);
      width: ${progress}%;
      transition: width 0.3s ease;
      box-shadow: 0 0 10px rgba(0, 255, 0, 0.5);
    `;
    
    const progressText = document.createElement('span');
    progressText.style.cssText = `
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      color: #00FF00;
      font-family: 'Courier Prime', 'Courier New', monospace;
      font-size: 0.75rem;
      text-shadow: 0 0 5px currentColor;
      z-index: 1;
    `;
    progressText.textContent = `${Math.round(progress)}%`;
    
    progressBar.appendChild(progressFill);
    progressBar.appendChild(progressText);
    container.appendChild(progressBar);
    
    return {
      element: progressBar,
      update: (newProgress) => {
        progressFill.style.width = `${newProgress}%`;
        progressText.textContent = `${Math.round(newProgress)}%`;
      }
    };
  }

  /**
   * Utility delay function
   * @param {number} ms - Milliseconds to delay
   */
  delay(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

  /**
   * Add Matrix-style focus effects to inputs
   * @param {HTMLElement} input - Input element
   */
  addInputEffect(input) {
    input.addEventListener('focus', () => {
      input.style.boxShadow = '0 0 0 2px #00FF00, 0 0 10px rgba(0, 255, 0, 0.3)';
      input.style.textShadow = '0 0 5px currentColor';
    });
    
    input.addEventListener('blur', () => {
      input.style.boxShadow = 'none';
      input.style.textShadow = '0 0 3px currentColor';
    });
  }

  /**
   * Create Matrix-style modal
   * @param {string} title - Modal title
   * @param {string} content - Modal content
   * @param {Array} buttons - Array of button objects {text, callback, type}
   */
  createModal(title, content, buttons = []) {
    const overlay = document.createElement('div');
    overlay.className = 'matrix-modal-overlay';
    overlay.style.cssText = `
      position: fixed;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      background: rgba(0, 0, 0, 0.8);
      display: flex;
      align-items: center;
      justify-content: center;
      z-index: 10000;
      backdrop-filter: blur(2px);
    `;
    
    const modal = document.createElement('div');
    modal.className = 'matrix-modal';
    modal.style.cssText = `
      background: #000000;
      border: 1px solid #00FF00;
      border-radius: 0.5rem;
      padding: 2rem;
      max-width: 500px;
      width: 90%;
      box-shadow: 0 0 20px rgba(0, 255, 0, 0.3);
      font-family: 'Courier Prime', 'Courier New', monospace;
      color: #00FF00;
    `;
    
    const titleElement = document.createElement('h3');
    titleElement.textContent = title;
    titleElement.style.cssText = `
      margin: 0 0 1rem 0;
      color: #00FF00;
      text-shadow: 0 0 10px currentColor;
    `;
    
    const contentElement = document.createElement('div');
    contentElement.innerHTML = content;
    contentElement.style.cssText = `
      margin-bottom: 2rem;
      line-height: 1.5;
    `;
    
    const buttonContainer = document.createElement('div');
    buttonContainer.style.cssText = `
      display: flex;
      gap: 1rem;
      justify-content: flex-end;
    `;
    
    buttons.forEach(button => {
      const btn = document.createElement('button');
      btn.textContent = button.text;
      btn.className = 'matrix-btn';
      btn.style.cssText = `
        background: transparent;
        border: 1px solid #008800;
        color: #00FF00;
        padding: 0.5rem 1rem;
        border-radius: 0.25rem;
        cursor: pointer;
        font-family: inherit;
        transition: all 0.3s ease;
      `;
      
      btn.addEventListener('click', () => {
        if (button.callback) button.callback();
        document.body.removeChild(overlay);
      });
      
      this.addButtonEffect(btn);
      buttonContainer.appendChild(btn);
    });
    
    modal.appendChild(titleElement);
    modal.appendChild(contentElement);
    modal.appendChild(buttonContainer);
    overlay.appendChild(modal);
    document.body.appendChild(overlay);
    
    // Close on overlay click
    overlay.addEventListener('click', (e) => {
      if (e.target === overlay) {
        document.body.removeChild(overlay);
      }
    });
    
    return overlay;
  }
}

// Create global instance
window.MatrixEffects = new MatrixEffects();

// Auto-initialize when DOM is ready
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', () => {
    window.MatrixEffects.init();
  });
} else {
  window.MatrixEffects.init();
}