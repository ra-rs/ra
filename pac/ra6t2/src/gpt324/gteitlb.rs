#[doc = "Register `GTEITLB` reader"]
pub struct R(crate::R<GTEITLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTEITLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTEITLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTEITLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTEITLB` writer"]
pub struct W(crate::W<GTEITLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTEITLB_SPEC>;
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
impl From<crate::W<GTEITLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTEITLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EBTLCA` reader - GTCCRA Register Buffer Transfer Extended Skipping Function Select"]
pub type EBTLCA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EBTLCA` writer - GTCCRA Register Buffer Transfer Extended Skipping Function Select"]
pub type EBTLCA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTEITLB_SPEC, u8, u8, 3, O>;
#[doc = "Field `EBTLCB` reader - GTCCRB Register Buffer Transfer Extended Skipping Function Select"]
pub type EBTLCB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EBTLCB` writer - GTCCRB Register Buffer Transfer Extended Skipping Function Select"]
pub type EBTLCB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTEITLB_SPEC, u8, u8, 3, O>;
#[doc = "Field `EBTLPR` reader - GTPR Register Buffer Transfer Extended Skipping Function Select"]
pub type EBTLPR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EBTLPR` writer - GTPR Register Buffer Transfer Extended Skipping Function Select"]
pub type EBTLPR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTEITLB_SPEC, u8, u8, 3, O>;
#[doc = "Field `EBTLADA` reader - GTADTRA Register Buffer Transfer Extended Skipping Function Select"]
pub type EBTLADA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EBTLADA` writer - GTADTRA Register Buffer Transfer Extended Skipping Function Select"]
pub type EBTLADA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTEITLB_SPEC, u8, u8, 3, O>;
#[doc = "Field `EBTLADB` reader - GTADTRB Register Buffer Transfer Extended Skipping Function Select"]
pub type EBTLADB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EBTLADB` writer - GTADTRB Register Buffer Transfer Extended Skipping Function Select"]
pub type EBTLADB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTEITLB_SPEC, u8, u8, 3, O>;
#[doc = "Field `EBTLDVU` reader - GTDVU Register Buffer Transfer Extended Skipping Function Select"]
pub type EBTLDVU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EBTLDVU` writer - GTDVU Register Buffer Transfer Extended Skipping Function Select"]
pub type EBTLDVU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTEITLB_SPEC, u8, u8, 3, O>;
#[doc = "Field `EBTLDVD` reader - GTDVD Register Buffer Transfer Extended Skipping Function Select"]
pub type EBTLDVD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EBTLDVD` writer - GTDVD Register Buffer Transfer Extended Skipping Function Select"]
pub type EBTLDVD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTEITLB_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - GTCCRA Register Buffer Transfer Extended Skipping Function Select"]
    #[inline(always)]
    pub fn ebtlca(&self) -> EBTLCA_R {
        EBTLCA_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - GTCCRB Register Buffer Transfer Extended Skipping Function Select"]
    #[inline(always)]
    pub fn ebtlcb(&self) -> EBTLCB_R {
        EBTLCB_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - GTPR Register Buffer Transfer Extended Skipping Function Select"]
    #[inline(always)]
    pub fn ebtlpr(&self) -> EBTLPR_R {
        EBTLPR_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - GTADTRA Register Buffer Transfer Extended Skipping Function Select"]
    #[inline(always)]
    pub fn ebtlada(&self) -> EBTLADA_R {
        EBTLADA_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - GTADTRB Register Buffer Transfer Extended Skipping Function Select"]
    #[inline(always)]
    pub fn ebtladb(&self) -> EBTLADB_R {
        EBTLADB_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - GTDVU Register Buffer Transfer Extended Skipping Function Select"]
    #[inline(always)]
    pub fn ebtldvu(&self) -> EBTLDVU_R {
        EBTLDVU_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - GTDVD Register Buffer Transfer Extended Skipping Function Select"]
    #[inline(always)]
    pub fn ebtldvd(&self) -> EBTLDVD_R {
        EBTLDVD_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - GTCCRA Register Buffer Transfer Extended Skipping Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn ebtlca(&mut self) -> EBTLCA_W<0> {
        EBTLCA_W::new(self)
    }
    #[doc = "Bits 4:6 - GTCCRB Register Buffer Transfer Extended Skipping Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn ebtlcb(&mut self) -> EBTLCB_W<4> {
        EBTLCB_W::new(self)
    }
    #[doc = "Bits 8:10 - GTPR Register Buffer Transfer Extended Skipping Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn ebtlpr(&mut self) -> EBTLPR_W<8> {
        EBTLPR_W::new(self)
    }
    #[doc = "Bits 16:18 - GTADTRA Register Buffer Transfer Extended Skipping Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn ebtlada(&mut self) -> EBTLADA_W<16> {
        EBTLADA_W::new(self)
    }
    #[doc = "Bits 20:22 - GTADTRB Register Buffer Transfer Extended Skipping Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn ebtladb(&mut self) -> EBTLADB_W<20> {
        EBTLADB_W::new(self)
    }
    #[doc = "Bits 24:26 - GTDVU Register Buffer Transfer Extended Skipping Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn ebtldvu(&mut self) -> EBTLDVU_W<24> {
        EBTLDVU_W::new(self)
    }
    #[doc = "Bits 28:30 - GTDVD Register Buffer Transfer Extended Skipping Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn ebtldvd(&mut self) -> EBTLDVD_W<28> {
        EBTLDVD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Extended Buffer Transfer Skipping Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gteitlb](index.html) module"]
pub struct GTEITLB_SPEC;
impl crate::RegisterSpec for GTEITLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gteitlb::R](R) reader structure"]
impl crate::Readable for GTEITLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gteitlb::W](W) writer structure"]
impl crate::Writable for GTEITLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTEITLB to value 0"]
impl crate::Resettable for GTEITLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
