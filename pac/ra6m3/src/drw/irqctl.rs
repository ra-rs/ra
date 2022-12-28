#[doc = "Register `IRQCTL` writer"]
pub struct W(crate::W<IRQCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IRQCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ENUMIRQ interrupt mask enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENUMIRQEN_AW {
    #[doc = "0: disable (mask) ENUMIRQ"]
    _0 = 0,
    #[doc = "1: enable (unmask) ENUMIRQ"]
    _1 = 1,
}
impl From<ENUMIRQEN_AW> for bool {
    #[inline(always)]
    fn from(variant: ENUMIRQEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENUMIRQEN` writer - ENUMIRQ interrupt mask enable"]
pub type ENUMIRQEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQCTL_SPEC, ENUMIRQEN_AW, O>;
impl<'a, const O: u8> ENUMIRQEN_W<'a, O> {
    #[doc = "disable (mask) ENUMIRQ"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENUMIRQEN_AW::_0)
    }
    #[doc = "enable (unmask) ENUMIRQ"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENUMIRQEN_AW::_1)
    }
}
#[doc = "DLISTIRQ interrupt mask enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLISTIRQEN_AW {
    #[doc = "0: disable (mask) DLISTIRQ"]
    _0 = 0,
    #[doc = "1: enable (unmask) DLISTIRQ"]
    _1 = 1,
}
impl From<DLISTIRQEN_AW> for bool {
    #[inline(always)]
    fn from(variant: DLISTIRQEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLISTIRQEN` writer - DLISTIRQ interrupt mask enable"]
pub type DLISTIRQEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQCTL_SPEC, DLISTIRQEN_AW, O>;
impl<'a, const O: u8> DLISTIRQEN_W<'a, O> {
    #[doc = "disable (mask) DLISTIRQ"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DLISTIRQEN_AW::_0)
    }
    #[doc = "enable (unmask) DLISTIRQ"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DLISTIRQEN_AW::_1)
    }
}
#[doc = "Clear enumeration interrupt ENUMIRQ\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENUMIRQCLR_AW {
    #[doc = "0: no ENUMIRQCLR clear"]
    _0 = 0,
    #[doc = "1: clear ENUMIRQCLR"]
    _1 = 1,
}
impl From<ENUMIRQCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: ENUMIRQCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENUMIRQCLR` writer - Clear enumeration interrupt ENUMIRQ"]
pub type ENUMIRQCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQCTL_SPEC, ENUMIRQCLR_AW, O>;
impl<'a, const O: u8> ENUMIRQCLR_W<'a, O> {
    #[doc = "no ENUMIRQCLR clear"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENUMIRQCLR_AW::_0)
    }
    #[doc = "clear ENUMIRQCLR"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENUMIRQCLR_AW::_1)
    }
}
#[doc = "Clear display list interrupt DLISTIRQ\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLISTIRQCLR_AW {
    #[doc = "0: no DLISTRQCLR clear"]
    _0 = 0,
    #[doc = "1: clear DLISTRQCLR"]
    _1 = 1,
}
impl From<DLISTIRQCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: DLISTIRQCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLISTIRQCLR` writer - Clear display list interrupt DLISTIRQ"]
pub type DLISTIRQCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQCTL_SPEC, DLISTIRQCLR_AW, O>;
impl<'a, const O: u8> DLISTIRQCLR_W<'a, O> {
    #[doc = "no DLISTRQCLR clear"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DLISTIRQCLR_AW::_0)
    }
    #[doc = "clear DLISTRQCLR"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DLISTIRQCLR_AW::_1)
    }
}
#[doc = "BUSIRQ interrupt mask enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSIRQEN_AW {
    #[doc = "0: disable (mask) BUSIRQ"]
    _0 = 0,
    #[doc = "1: enable (unmask) BUSIRQ"]
    _1 = 1,
}
impl From<BUSIRQEN_AW> for bool {
    #[inline(always)]
    fn from(variant: BUSIRQEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSIRQEN` writer - BUSIRQ interrupt mask enable"]
pub type BUSIRQEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQCTL_SPEC, BUSIRQEN_AW, O>;
impl<'a, const O: u8> BUSIRQEN_W<'a, O> {
    #[doc = "disable (mask) BUSIRQ"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUSIRQEN_AW::_0)
    }
    #[doc = "enable (unmask) BUSIRQ"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUSIRQEN_AW::_1)
    }
}
#[doc = "Clear bus error interrupt BUSIRQ\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSIRQCLR_AW {
    #[doc = "0: no BUSIRQCLR clear"]
    _0 = 0,
    #[doc = "1: clear BUSIRQCLR"]
    _1 = 1,
}
impl From<BUSIRQCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: BUSIRQCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSIRQCLR` writer - Clear bus error interrupt BUSIRQ"]
pub type BUSIRQCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQCTL_SPEC, BUSIRQCLR_AW, O>;
impl<'a, const O: u8> BUSIRQCLR_W<'a, O> {
    #[doc = "no BUSIRQCLR clear"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUSIRQCLR_AW::_0)
    }
    #[doc = "clear BUSIRQCLR"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUSIRQCLR_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - ENUMIRQ interrupt mask enable"]
    #[inline(always)]
    #[must_use]
    pub fn enumirqen(&mut self) -> ENUMIRQEN_W<0> {
        ENUMIRQEN_W::new(self)
    }
    #[doc = "Bit 1 - DLISTIRQ interrupt mask enable"]
    #[inline(always)]
    #[must_use]
    pub fn dlistirqen(&mut self) -> DLISTIRQEN_W<1> {
        DLISTIRQEN_W::new(self)
    }
    #[doc = "Bit 2 - Clear enumeration interrupt ENUMIRQ"]
    #[inline(always)]
    #[must_use]
    pub fn enumirqclr(&mut self) -> ENUMIRQCLR_W<2> {
        ENUMIRQCLR_W::new(self)
    }
    #[doc = "Bit 3 - Clear display list interrupt DLISTIRQ"]
    #[inline(always)]
    #[must_use]
    pub fn dlistirqclr(&mut self) -> DLISTIRQCLR_W<3> {
        DLISTIRQCLR_W::new(self)
    }
    #[doc = "Bit 4 - BUSIRQ interrupt mask enable"]
    #[inline(always)]
    #[must_use]
    pub fn busirqen(&mut self) -> BUSIRQEN_W<4> {
        BUSIRQEN_W::new(self)
    }
    #[doc = "Bit 5 - Clear bus error interrupt BUSIRQ"]
    #[inline(always)]
    #[must_use]
    pub fn busirqclr(&mut self) -> BUSIRQCLR_W<5> {
        BUSIRQCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqctl](index.html) module"]
pub struct IRQCTL_SPEC;
impl crate::RegisterSpec for IRQCTL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [irqctl::W](W) writer structure"]
impl crate::Writable for IRQCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRQCTL to value 0"]
impl crate::Resettable for IRQCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
