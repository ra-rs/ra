#[doc = "Register `ICR` reader"]
pub struct R(crate::R<ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BFDIE` reader - Break Field Low Width Detected Interrupt Enable"]
pub type BFDIE_R = crate::BitReader<BFDIE_A>;
#[doc = "Break Field Low Width Detected Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFDIE_A {
    #[doc = "0: Interrupts on detection of the low width for a Break Field are disabled."]
    _0 = 0,
    #[doc = "1: Interrupts on detection of the low width for a Break Field are enabled."]
    _1 = 1,
}
impl From<BFDIE_A> for bool {
    #[inline(always)]
    fn from(variant: BFDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl BFDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFDIE_A {
        match self.bits {
            false => BFDIE_A::_0,
            true => BFDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BFDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BFDIE_A::_1
    }
}
#[doc = "Field `BFDIE` writer - Break Field Low Width Detected Interrupt Enable"]
pub type BFDIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICR_SPEC, BFDIE_A, O>;
impl<'a, const O: u8> BFDIE_W<'a, O> {
    #[doc = "Interrupts on detection of the low width for a Break Field are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BFDIE_A::_0)
    }
    #[doc = "Interrupts on detection of the low width for a Break Field are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BFDIE_A::_1)
    }
}
#[doc = "Field `CF0MIE` reader - Control Field 0 Match Detected Interrupt Enable"]
pub type CF0MIE_R = crate::BitReader<CF0MIE_A>;
#[doc = "Control Field 0 Match Detected Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CF0MIE_A {
    #[doc = "0: Interrupts on detection of a match with Control Field 0 are disabled."]
    _0 = 0,
    #[doc = "1: Interrupts on detection of a match with Control Field 0 are enabled."]
    _1 = 1,
}
impl From<CF0MIE_A> for bool {
    #[inline(always)]
    fn from(variant: CF0MIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CF0MIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CF0MIE_A {
        match self.bits {
            false => CF0MIE_A::_0,
            true => CF0MIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CF0MIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CF0MIE_A::_1
    }
}
#[doc = "Field `CF0MIE` writer - Control Field 0 Match Detected Interrupt Enable"]
pub type CF0MIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICR_SPEC, CF0MIE_A, O>;
impl<'a, const O: u8> CF0MIE_W<'a, O> {
    #[doc = "Interrupts on detection of a match with Control Field 0 are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CF0MIE_A::_0)
    }
    #[doc = "Interrupts on detection of a match with Control Field 0 are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CF0MIE_A::_1)
    }
}
#[doc = "Field `CF1MIE` reader - Control Field 1 Match Detected Interrupt Enable"]
pub type CF1MIE_R = crate::BitReader<CF1MIE_A>;
#[doc = "Control Field 1 Match Detected Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CF1MIE_A {
    #[doc = "0: Interrupts on detection of a match with Control Field 1 are disabled."]
    _0 = 0,
    #[doc = "1: Interrupts on detection of a match with Control Field 1 are enabled."]
    _1 = 1,
}
impl From<CF1MIE_A> for bool {
    #[inline(always)]
    fn from(variant: CF1MIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CF1MIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CF1MIE_A {
        match self.bits {
            false => CF1MIE_A::_0,
            true => CF1MIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CF1MIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CF1MIE_A::_1
    }
}
#[doc = "Field `CF1MIE` writer - Control Field 1 Match Detected Interrupt Enable"]
pub type CF1MIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICR_SPEC, CF1MIE_A, O>;
impl<'a, const O: u8> CF1MIE_W<'a, O> {
    #[doc = "Interrupts on detection of a match with Control Field 1 are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CF1MIE_A::_0)
    }
    #[doc = "Interrupts on detection of a match with Control Field 1 are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CF1MIE_A::_1)
    }
}
#[doc = "Field `PIBDIE` reader - Priority Interrupt Bit Detected Interrupt Enable"]
pub type PIBDIE_R = crate::BitReader<PIBDIE_A>;
#[doc = "Priority Interrupt Bit Detected Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIBDIE_A {
    #[doc = "0: Interrupts on detection of the priority interrupt bit are disabled."]
    _0 = 0,
    #[doc = "1: Interrupts on detection of the priority interrupt bit are enabled."]
    _1 = 1,
}
impl From<PIBDIE_A> for bool {
    #[inline(always)]
    fn from(variant: PIBDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl PIBDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIBDIE_A {
        match self.bits {
            false => PIBDIE_A::_0,
            true => PIBDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIBDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIBDIE_A::_1
    }
}
#[doc = "Field `PIBDIE` writer - Priority Interrupt Bit Detected Interrupt Enable"]
pub type PIBDIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICR_SPEC, PIBDIE_A, O>;
impl<'a, const O: u8> PIBDIE_W<'a, O> {
    #[doc = "Interrupts on detection of the priority interrupt bit are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIBDIE_A::_0)
    }
    #[doc = "Interrupts on detection of the priority interrupt bit are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIBDIE_A::_1)
    }
}
#[doc = "Field `BCDIE` reader - Bus Collision Detected Interrupt Enable"]
pub type BCDIE_R = crate::BitReader<BCDIE_A>;
#[doc = "Bus Collision Detected Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCDIE_A {
    #[doc = "0: Interrupts on detection of a bus collision are disabled."]
    _0 = 0,
    #[doc = "1: Interrupts on detection of a bus collision are enabled."]
    _1 = 1,
}
impl From<BCDIE_A> for bool {
    #[inline(always)]
    fn from(variant: BCDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl BCDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCDIE_A {
        match self.bits {
            false => BCDIE_A::_0,
            true => BCDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BCDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BCDIE_A::_1
    }
}
#[doc = "Field `BCDIE` writer - Bus Collision Detected Interrupt Enable"]
pub type BCDIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICR_SPEC, BCDIE_A, O>;
impl<'a, const O: u8> BCDIE_W<'a, O> {
    #[doc = "Interrupts on detection of a bus collision are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BCDIE_A::_0)
    }
    #[doc = "Interrupts on detection of a bus collision are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BCDIE_A::_1)
    }
}
#[doc = "Field `AEDIE` reader - Valid Edge Detected Interrupt Enable"]
pub type AEDIE_R = crate::BitReader<AEDIE_A>;
#[doc = "Valid Edge Detected Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AEDIE_A {
    #[doc = "0: Interrupts on detection of a valid edge are disabled."]
    _0 = 0,
    #[doc = "1: Interrupts on detection of a valid edge are enabled."]
    _1 = 1,
}
impl From<AEDIE_A> for bool {
    #[inline(always)]
    fn from(variant: AEDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl AEDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AEDIE_A {
        match self.bits {
            false => AEDIE_A::_0,
            true => AEDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AEDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AEDIE_A::_1
    }
}
#[doc = "Field `AEDIE` writer - Valid Edge Detected Interrupt Enable"]
pub type AEDIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICR_SPEC, AEDIE_A, O>;
impl<'a, const O: u8> AEDIE_W<'a, O> {
    #[doc = "Interrupts on detection of a valid edge are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AEDIE_A::_0)
    }
    #[doc = "Interrupts on detection of a valid edge are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AEDIE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Break Field Low Width Detected Interrupt Enable"]
    #[inline(always)]
    pub fn bfdie(&self) -> BFDIE_R {
        BFDIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Control Field 0 Match Detected Interrupt Enable"]
    #[inline(always)]
    pub fn cf0mie(&self) -> CF0MIE_R {
        CF0MIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Control Field 1 Match Detected Interrupt Enable"]
    #[inline(always)]
    pub fn cf1mie(&self) -> CF1MIE_R {
        CF1MIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Priority Interrupt Bit Detected Interrupt Enable"]
    #[inline(always)]
    pub fn pibdie(&self) -> PIBDIE_R {
        PIBDIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bus Collision Detected Interrupt Enable"]
    #[inline(always)]
    pub fn bcdie(&self) -> BCDIE_R {
        BCDIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Valid Edge Detected Interrupt Enable"]
    #[inline(always)]
    pub fn aedie(&self) -> AEDIE_R {
        AEDIE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Break Field Low Width Detected Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bfdie(&mut self) -> BFDIE_W<0> {
        BFDIE_W::new(self)
    }
    #[doc = "Bit 1 - Control Field 0 Match Detected Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cf0mie(&mut self) -> CF0MIE_W<1> {
        CF0MIE_W::new(self)
    }
    #[doc = "Bit 2 - Control Field 1 Match Detected Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cf1mie(&mut self) -> CF1MIE_W<2> {
        CF1MIE_W::new(self)
    }
    #[doc = "Bit 3 - Priority Interrupt Bit Detected Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pibdie(&mut self) -> PIBDIE_W<3> {
        PIBDIE_W::new(self)
    }
    #[doc = "Bit 4 - Bus Collision Detected Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bcdie(&mut self) -> BCDIE_W<4> {
        BCDIE_W::new(self)
    }
    #[doc = "Bit 5 - Valid Edge Detected Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aedie(&mut self) -> AEDIE_W<5> {
        AEDIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [icr::R](R) reader structure"]
impl crate::Readable for ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
