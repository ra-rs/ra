#[doc = "Register `PFENET` reader"]
pub struct R(crate::R<PFENET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFENET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFENET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFENET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PFENET` writer"]
pub struct W(crate::W<PFENET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFENET_SPEC>;
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
impl From<crate::W<PFENET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFENET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHYMODE0` reader - Ethernet Mode Setting ch0"]
pub type PHYMODE0_R = crate::BitReader<PHYMODE0_A>;
#[doc = "Ethernet Mode Setting ch0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PHYMODE0_A {
    #[doc = "0: RMII mode (ETHERC channel 0)"]
    _0 = 0,
    #[doc = "1: MII mode (ETHERC channel 0)"]
    _1 = 1,
}
impl From<PHYMODE0_A> for bool {
    #[inline(always)]
    fn from(variant: PHYMODE0_A) -> Self {
        variant as u8 != 0
    }
}
impl PHYMODE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHYMODE0_A {
        match self.bits {
            false => PHYMODE0_A::_0,
            true => PHYMODE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PHYMODE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PHYMODE0_A::_1
    }
}
#[doc = "Field `PHYMODE0` writer - Ethernet Mode Setting ch0"]
pub type PHYMODE0_W<'a, const O: u8> = crate::BitWriter<'a, u8, PFENET_SPEC, PHYMODE0_A, O>;
impl<'a, const O: u8> PHYMODE0_W<'a, O> {
    #[doc = "RMII mode (ETHERC channel 0)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PHYMODE0_A::_0)
    }
    #[doc = "MII mode (ETHERC channel 0)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PHYMODE0_A::_1)
    }
}
impl R {
    #[doc = "Bit 4 - Ethernet Mode Setting ch0"]
    #[inline(always)]
    pub fn phymode0(&self) -> PHYMODE0_R {
        PHYMODE0_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Ethernet Mode Setting ch0"]
    #[inline(always)]
    #[must_use]
    pub fn phymode0(&mut self) -> PHYMODE0_W<4> {
        PHYMODE0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfenet](index.html) module"]
pub struct PFENET_SPEC;
impl crate::RegisterSpec for PFENET_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pfenet::R](R) reader structure"]
impl crate::Readable for PFENET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pfenet::W](W) writer structure"]
impl crate::Writable for PFENET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PFENET to value 0"]
impl crate::Resettable for PFENET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
