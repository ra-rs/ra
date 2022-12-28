#[doc = "Register `INTENB1` reader"]
pub struct R(crate::R<INTENB1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENB1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENB1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENB1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENB1` writer"]
pub struct W(crate::W<INTENB1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENB1_SPEC>;
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
impl From<crate::W<INTENB1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENB1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDDETINTE0` reader - PDDETINT0 Detection Interrupt Enable"]
pub type PDDETINTE0_R = crate::BitReader<PDDETINTE0_A>;
#[doc = "PDDETINT0 Detection Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDDETINTE0_A {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<PDDETINTE0_A> for bool {
    #[inline(always)]
    fn from(variant: PDDETINTE0_A) -> Self {
        variant as u8 != 0
    }
}
impl PDDETINTE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDDETINTE0_A {
        match self.bits {
            false => PDDETINTE0_A::_0,
            true => PDDETINTE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDDETINTE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDDETINTE0_A::_1
    }
}
#[doc = "Field `PDDETINTE0` writer - PDDETINT0 Detection Interrupt Enable"]
pub type PDDETINTE0_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTENB1_SPEC, PDDETINTE0_A, O>;
impl<'a, const O: u8> PDDETINTE0_W<'a, O> {
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDDETINTE0_A::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDDETINTE0_A::_1)
    }
}
#[doc = "Field `SACKE` reader - Setup Transaction Normal Response Interrupt Enable"]
pub type SACKE_R = crate::BitReader<SACKE_A>;
#[doc = "Setup Transaction Normal Response Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SACKE_A {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<SACKE_A> for bool {
    #[inline(always)]
    fn from(variant: SACKE_A) -> Self {
        variant as u8 != 0
    }
}
impl SACKE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SACKE_A {
        match self.bits {
            false => SACKE_A::_0,
            true => SACKE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SACKE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SACKE_A::_1
    }
}
#[doc = "Field `SACKE` writer - Setup Transaction Normal Response Interrupt Enable"]
pub type SACKE_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTENB1_SPEC, SACKE_A, O>;
impl<'a, const O: u8> SACKE_W<'a, O> {
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SACKE_A::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SACKE_A::_1)
    }
}
#[doc = "Field `SIGNE` reader - Setup Transaction Error Interrupt Enable"]
pub type SIGNE_R = crate::BitReader<SIGNE_A>;
#[doc = "Setup Transaction Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIGNE_A {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<SIGNE_A> for bool {
    #[inline(always)]
    fn from(variant: SIGNE_A) -> Self {
        variant as u8 != 0
    }
}
impl SIGNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIGNE_A {
        match self.bits {
            false => SIGNE_A::_0,
            true => SIGNE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SIGNE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SIGNE_A::_1
    }
}
#[doc = "Field `SIGNE` writer - Setup Transaction Error Interrupt Enable"]
pub type SIGNE_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTENB1_SPEC, SIGNE_A, O>;
impl<'a, const O: u8> SIGNE_W<'a, O> {
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SIGNE_A::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SIGNE_A::_1)
    }
}
#[doc = "Field `EOFERRE` reader - EOF Error Detection Interrupt Enable"]
pub type EOFERRE_R = crate::BitReader<EOFERRE_A>;
#[doc = "EOF Error Detection Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOFERRE_A {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<EOFERRE_A> for bool {
    #[inline(always)]
    fn from(variant: EOFERRE_A) -> Self {
        variant as u8 != 0
    }
}
impl EOFERRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOFERRE_A {
        match self.bits {
            false => EOFERRE_A::_0,
            true => EOFERRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOFERRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOFERRE_A::_1
    }
}
#[doc = "Field `EOFERRE` writer - EOF Error Detection Interrupt Enable"]
pub type EOFERRE_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTENB1_SPEC, EOFERRE_A, O>;
impl<'a, const O: u8> EOFERRE_W<'a, O> {
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOFERRE_A::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOFERRE_A::_1)
    }
}
#[doc = "Field `ATTCHE` reader - Connection Detection Interrupt Enable"]
pub type ATTCHE_R = crate::BitReader<ATTCHE_A>;
#[doc = "Connection Detection Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ATTCHE_A {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<ATTCHE_A> for bool {
    #[inline(always)]
    fn from(variant: ATTCHE_A) -> Self {
        variant as u8 != 0
    }
}
impl ATTCHE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ATTCHE_A {
        match self.bits {
            false => ATTCHE_A::_0,
            true => ATTCHE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ATTCHE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ATTCHE_A::_1
    }
}
#[doc = "Field `ATTCHE` writer - Connection Detection Interrupt Enable"]
pub type ATTCHE_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTENB1_SPEC, ATTCHE_A, O>;
impl<'a, const O: u8> ATTCHE_W<'a, O> {
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ATTCHE_A::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ATTCHE_A::_1)
    }
}
#[doc = "Field `DTCHE` reader - Disconnection Detection Interrupt Enable"]
pub type DTCHE_R = crate::BitReader<DTCHE_A>;
#[doc = "Disconnection Detection Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTCHE_A {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<DTCHE_A> for bool {
    #[inline(always)]
    fn from(variant: DTCHE_A) -> Self {
        variant as u8 != 0
    }
}
impl DTCHE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTCHE_A {
        match self.bits {
            false => DTCHE_A::_0,
            true => DTCHE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTCHE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTCHE_A::_1
    }
}
#[doc = "Field `DTCHE` writer - Disconnection Detection Interrupt Enable"]
pub type DTCHE_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTENB1_SPEC, DTCHE_A, O>;
impl<'a, const O: u8> DTCHE_W<'a, O> {
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTCHE_A::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTCHE_A::_1)
    }
}
#[doc = "Field `BCHGE` reader - USB Bus Change Interrupt Enable"]
pub type BCHGE_R = crate::BitReader<BCHGE_A>;
#[doc = "USB Bus Change Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCHGE_A {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<BCHGE_A> for bool {
    #[inline(always)]
    fn from(variant: BCHGE_A) -> Self {
        variant as u8 != 0
    }
}
impl BCHGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCHGE_A {
        match self.bits {
            false => BCHGE_A::_0,
            true => BCHGE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BCHGE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BCHGE_A::_1
    }
}
#[doc = "Field `BCHGE` writer - USB Bus Change Interrupt Enable"]
pub type BCHGE_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTENB1_SPEC, BCHGE_A, O>;
impl<'a, const O: u8> BCHGE_W<'a, O> {
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BCHGE_A::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BCHGE_A::_1)
    }
}
#[doc = "Field `OVRCRE` reader - Overcurrent Input Change Interrupt Enable"]
pub type OVRCRE_R = crate::BitReader<OVRCRE_A>;
#[doc = "Overcurrent Input Change Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRCRE_A {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<OVRCRE_A> for bool {
    #[inline(always)]
    fn from(variant: OVRCRE_A) -> Self {
        variant as u8 != 0
    }
}
impl OVRCRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRCRE_A {
        match self.bits {
            false => OVRCRE_A::_0,
            true => OVRCRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVRCRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVRCRE_A::_1
    }
}
#[doc = "Field `OVRCRE` writer - Overcurrent Input Change Interrupt Enable"]
pub type OVRCRE_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTENB1_SPEC, OVRCRE_A, O>;
impl<'a, const O: u8> OVRCRE_W<'a, O> {
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVRCRE_A::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVRCRE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - PDDETINT0 Detection Interrupt Enable"]
    #[inline(always)]
    pub fn pddetinte0(&self) -> PDDETINTE0_R {
        PDDETINTE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Setup Transaction Normal Response Interrupt Enable"]
    #[inline(always)]
    pub fn sacke(&self) -> SACKE_R {
        SACKE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Setup Transaction Error Interrupt Enable"]
    #[inline(always)]
    pub fn signe(&self) -> SIGNE_R {
        SIGNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EOF Error Detection Interrupt Enable"]
    #[inline(always)]
    pub fn eoferre(&self) -> EOFERRE_R {
        EOFERRE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 11 - Connection Detection Interrupt Enable"]
    #[inline(always)]
    pub fn attche(&self) -> ATTCHE_R {
        ATTCHE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Disconnection Detection Interrupt Enable"]
    #[inline(always)]
    pub fn dtche(&self) -> DTCHE_R {
        DTCHE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USB Bus Change Interrupt Enable"]
    #[inline(always)]
    pub fn bchge(&self) -> BCHGE_R {
        BCHGE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Overcurrent Input Change Interrupt Enable"]
    #[inline(always)]
    pub fn ovrcre(&self) -> OVRCRE_R {
        OVRCRE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDDETINT0 Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pddetinte0(&mut self) -> PDDETINTE0_W<0> {
        PDDETINTE0_W::new(self)
    }
    #[doc = "Bit 4 - Setup Transaction Normal Response Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sacke(&mut self) -> SACKE_W<4> {
        SACKE_W::new(self)
    }
    #[doc = "Bit 5 - Setup Transaction Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn signe(&mut self) -> SIGNE_W<5> {
        SIGNE_W::new(self)
    }
    #[doc = "Bit 6 - EOF Error Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eoferre(&mut self) -> EOFERRE_W<6> {
        EOFERRE_W::new(self)
    }
    #[doc = "Bit 11 - Connection Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn attche(&mut self) -> ATTCHE_W<11> {
        ATTCHE_W::new(self)
    }
    #[doc = "Bit 12 - Disconnection Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtche(&mut self) -> DTCHE_W<12> {
        DTCHE_W::new(self)
    }
    #[doc = "Bit 14 - USB Bus Change Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bchge(&mut self) -> BCHGE_W<14> {
        BCHGE_W::new(self)
    }
    #[doc = "Bit 15 - Overcurrent Input Change Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovrcre(&mut self) -> OVRCRE_W<15> {
        OVRCRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenb1](index.html) module"]
pub struct INTENB1_SPEC;
impl crate::RegisterSpec for INTENB1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [intenb1::R](R) reader structure"]
impl crate::Readable for INTENB1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenb1::W](W) writer structure"]
impl crate::Writable for INTENB1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENB1 to value 0"]
impl crate::Resettable for INTENB1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
